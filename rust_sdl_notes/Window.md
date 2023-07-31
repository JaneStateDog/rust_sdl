For now, I will program the window as if you can only have one `Window` per `Engine`, because I am lazy. I would like to eventually allow you to have more than one window running at a time, assuming this is even possible. 

Every [[Window]] has a [[Surface]]. This [[Surface]] is what is being drawn to the window.

When creating a window, you need to pass the [[Engine]] so we can get the `sdl_context`.