Mouse Jiggler

A simple Rust utility that simulates small mouse movements (“jiggles”) after a specified period of user inactivity to prevent the system from going idle or activating screen savers.
Features

    Detects user inactivity (no keyboard or mouse input)

    Moves the mouse cursor slightly to keep the system active

    Configurable inactivity timeout before jiggle starts

    Cross-platform support focused on macOS and Linux

How to Use
Build

Make sure you have Rust installed. Then clone and build the project:

git clone https://github.com/yourusername/mouse-jiggler.git
cd mouse-jiggler
cargo build --release

Run

Run the executable with an optional inactivity timeout in seconds (default is 300 seconds):

./target/release/mouse-jiggler --timeout 300

Command Line Options

    --timeout <seconds> — Time of inactivity before mouse jiggle begins (default 300 seconds)

How It Works

    The program monitors system input activity.

    After the configured timeout of inactivity, it jiggles the mouse cursor by a few pixels every few seconds.

    Jiggle stops when user input is detected again.

