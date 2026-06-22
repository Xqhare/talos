- [ ] (A) State rework :: While writing ananke, I found the current state system to be lacking in ergonomics especially. (Looking at the recursive state management needed for displaying the table)
    * Just `Box` them and be done with it I think
    - [ ] State doc :: All `States` need better doc and a `new` function
- [ ] (A) Update Readme :: Add screenshot of ananke @README.md
- [ ] Bug fixes & improvements ::
    - [ ] Incorrectly rendered styling :: Sometimes cells are not rendered with the correct styling / symbols are drawn at the wrong coords (always left top in row 0) when scrolling a table
    - [ ] Textbox Cursor :: Cursor of TextBoxes is not always correct (for a large box, the cursor will be at the bottom left corner, no matter the text layout)
    - [ ] Secondary styles :: (like clicked style for button) need to be optional and default to the set style using `style()`, not default to terminal default
    - [ ] Debugger :: Some way for `printf` debugging
        * Lightweight logger basically, to write the output to disk, instead of relying on stdout
- [C] ~~Windows support :: If I ever feel the desire to work with the windows kernel again~~ 
- [C] ~~Double width :: Add 16 Code pages for double wide characters (with 16 already reserved for default single width == 32 Code pages out of 256 reserved => 224 free)~~
    * ~~With support for rendering them~~
- [ ] (Z) Movement tracking :: Mouse position reporting without a mouse button pressed
    - [ ] Backend :: 
        - [x] Xterm Parser :: 
        - [ ] System flags :: to enable mouse reporting in `TerminalIO`
            * Needs to be configurable
    - [x] Frontend :: 
- [ ] (Z) Image renderer :: 
	* needs a `png` and `jpeg` reader util, then I could average adjoining pixels until I reach a target resolution (area passed into render?)
- [ ] Widgets :: 
    - [ ] Simple Widgets :: 
        - [ ] Key hints ::
            * E.g "F1 -> Help | ESC -> Exit"
    - [ ] Stateful Widgets :: 
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
