mod rect;
pub use rect::Rect;

mod direction;
pub use direction::Direction;

mod point;
pub use point::Point;

mod constraint;
pub use constraint::Constraint;

pub struct Layout {
    direction: Direction,
    constraints: Vec<Constraint>,
    margin: u16,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            constraints: Vec::new(),
            margin: 0,
        }
    }
}

impl Layout {
    #[must_use]
    pub fn new(direction: Direction, constraints: Vec<Constraint>, margin: u16) -> Layout {
        Layout {
            direction,
            constraints,
            margin,
        }
    }

    // Using a heap allocation for the return here is fine.
    /// Splits the given area into smaller Rects based on the layout constraints.
    #[must_use]
    pub fn split(&self, area: Rect) -> Vec<Rect> {
        // 1. Apply Margin
        // If margin is too big, return empty rects or handling gracefully?
        // Here we just shrink to 0 if margin consumes everything.
        let inner_area = {
            let double_margin = self.margin.saturating_mul(2);
            if area.width <= double_margin || area.height <= double_margin {
                return vec![Rect::default(); self.constraints.len()];
            }
            Rect {
                x: area.x + self.margin,
                y: area.y + self.margin,
                width: area.width - double_margin,
                height: area.height - double_margin,
            }
        };

        // 2. Identify available space along the main axis
        let total_space = match self.direction {
            Direction::Horizontal => inner_area.width,
            Direction::Vertical => inner_area.height,
        };

        // 3. Solve the Constraints to get a list of lengths (u16)
        let sizes = self.solve_constraints(total_space);

        // 4. Convert lengths into Rects
        let mut rects = Vec::with_capacity(self.constraints.len());
        let mut current_pos = match self.direction {
            Direction::Horizontal => inner_area.x,
            Direction::Vertical => inner_area.y,
        };

        for size in sizes {
            let rect = match self.direction {
                Direction::Horizontal => Rect {
                    x: current_pos,
                    y: inner_area.y,
                    width: size,
                    height: inner_area.height,
                },
                Direction::Vertical => Rect {
                    x: inner_area.x,
                    y: current_pos,
                    width: inner_area.width,
                    height: size,
                },
            };
            rects.push(rect);
            current_pos = current_pos.saturating_add(size);
        }

        rects
    }

    fn solve_constraints(&self, total_space: u16) -> Vec<u16> {
        let mut results = vec![0; self.constraints.len()];
        let mut used_space: u16 = 0;
        let mut flex_count: u16 = 0;

        // Pass 1: Calculate Fixed sizes (Length, Percentage, Ratio)
        for (i, constraint) in self.constraints.iter().enumerate() {
            match constraint {
                Constraint::Length(len) => {
                    let size = *len;
                    results[i] = size;
                    used_space = used_space.saturating_add(size);
                }
                Constraint::Percentage(p) => {
                    // Simple integer math: total * p / 100
                    let size = (u32::from(total_space) * u32::from(*p) / 100) as u16;
                    results[i] = size;
                    used_space = used_space.saturating_add(size);
                }
                Constraint::Ratio(num, den) => {
                    if *den == 0 {
                        results[i] = 0;
                    } else {
                        let size = (u32::from(total_space) * *num / *den) as u16;
                        results[i] = size;
                        used_space = used_space.saturating_add(size);
                    }
                }
                Constraint::Min(_) => {
                    flex_count += 1;
                }
                Constraint::Max(max) => {
                    // Let's treat Max as a fixed claim for now.
                    // Simple approach: Treat Max as Length for initial allocation.
                    let size = *max;
                    results[i] = size;
                    used_space = used_space.saturating_add(size);
                }
            }
        }

        // Pass 2: Distribute Remaining Space to Flex (Min) items
        // `Min` acts as "Fill the rest"
        if flex_count > 0 {
            let remaining = total_space.saturating_sub(used_space);
            let per_flex = remaining / flex_count;
            let mut remainder = remaining % flex_count;

            for (i, constraint) in self.constraints.iter().enumerate() {
                if let Constraint::Min(min_req) = constraint {
                    let mut size = per_flex;
                    // Distribute the rounding error pixels
                    if remainder > 0 {
                        size += 1;
                        remainder -= 1;
                    }

                    size = std::cmp::max(size, *min_req);

                    results[i] = size;
                }
            }
        }

        results
    }
}
