- [ ] Key hints ::
    * E.g "F1 -> Help | ESC -> Exit"
- [ ] Context Menu :: 
    * Really just a menu button drawn at a specific set of coords (wherever the mouse clicked
- [ ] (C) *Tree View* :: 
    * It handles expanding/collapsing nodes (e.g., ▼ Folder, ▶ Subfolder).
- [ ] (C) *Notification* :: 
    * Really just a Block with a title and a single Text widget inside
    * Timer needed / also alternate way of dismissing
    * Persistant vs temporary -> Both would be nice
- [ ] (C) *Separator* :: 
    * horizontal or vertical
    * custom symbol
- [ ] Slider :: 
- [ ] Radio Button :: 
- [ ] Multi-line text box :: 
- [x] (C) *Dropdown selection* :: 
- [ ] Tabs :: 
    * A widget that allows the user to switch between tabs / Windows / Panes whatever
- [ ] (C) *ScrollArea* :: 
    * Both vertical and horizontal scrollable
- [ ] Colour picker :: 
    * A widget that allows the user to pick a colour
- [ ] Date and Time picker :: 
    * Calendar and Clock
- [ ] Gauge (like a speedometer) :: hard because round things are hard, but also interesting because of it
    * ▅,▙ etc
- [ ] Modal Dialog :: 
    * A widget that blocks input from the user until it is closed
    * Would also shadow the rest of the screen / run dim as the styles
    * Draw itself on top of the rest of the screen
- [ ] Chart widget :: 
    - [ ] (C) *Sparkline* :: One line high chart using varying block heights ( , ▂, ▃, ▄, ▅, ▆, ▇, █ )
    - [ ] Column / Bar :: 
    - [ ] Stacked Column / Bar :: 
    - [ ] Min - Max Chart :: (Two points per x coordinate, connected with vertical lines)
    - [ ] Line :: (Only points. No interconnecting lines)
    * Support for `usize`, `isize` and `f32` & `f64`
    * `isize` and `float` will need the x-axis to be in the middle to support negative values
