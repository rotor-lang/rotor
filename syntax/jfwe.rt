use gui [window, event];

template windowData {
    title: str = "New window";
    x, y: pair<i16> = [640, 480];
}

template playerData {
    name: str = "Danny";
    x, y: pair<i16> = [0, 0]; // Center
    inventory: array<str>[10];
}

fn createWindow(data: windowData): gui.window {
    let window = gui.window(data.title, data.x, data.y);
    window.create();
    return window;
}

fn drawPlayer(*window: gui.window, player: playerData) {
    window.draw.image("danny.png", player.x, player.y);
}

fn main(): i32 {
    gui.init();
    let game = createWindow();
    let event = gui.event;
    loop {
        event.getEvent();
        if (event == gui.event.keyPressed && gui.event.details.key == gui.key.ESC) {
            window.terminate();
        }
        drawPlayer(game);
        game.flush();
    }
    return 0;
}