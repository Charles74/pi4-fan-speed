# pi4-fan-speed
A tiny rust application for Raspberry Pi 4 to control a 5v 
fan's speed based on the CPU temperature.

The service will log the temperature and duty cycle 15 seconds
after first startup, then it will log it again every 15 minutes.
The fan speed is adjusted every 5 seconds

# Note
The Rpi4 cannot drive the fan directly, you will need additional components to achieve this.

In my case I used a KSP06 (MPSA06) NPN transistor, 1N4148 diode, and a 1.5k ohm resistor.
For the transistor any general purpose NPN transistor will do, most people use a 2N2222

Here is the circuit diagram.

![Circuit Diagram](https://github.com/charles74/pi4-fan-speed/blob/main/circuit.png?raw=true)

If soldering this simple circuit onto some vero board or 
perf board is too far out-side of your comfort zone,
you can get an off the shelf solution like: https://thepihut.com/products/fan-controller-for-raspberry-pi

# Why Rust
Honestly, I haven't got a good answer, other than I wanted
a small memory footprint. And the rust app
did the trick.

While I am confessing, ChatGPT wrote most of the app.

I actually started with a C++ app then it hit me like a ton 
of bricks, I haven't used C++ in 15 years and I recall nothing.
Then I thought Rust... and it is way different so ChatGPT and I
got going on this little app.

# Compile

```shell
cargo build --release
```

# Install

```shell
sudo cp target/release/pi-fan-speed /usr/local/bin
```

# Configure

```shell
cd scripts
```
Edit the file `pi-fan-speed.service` and update the line 
containing `User=<your user>` and set the user, you want the 
fan service to run as. I just used my username on the pi

Then install the systemd service.
```shell
./install.sh
```
Use journalctl to view the log for the service and after 15 
seconds you should see the first log message.

```shell
journalctl -u pi-fan-speed -f
Nov 02 16:51:57 storage systemd[1]: Started pi-fan-speed.service - Fan Speed Control Service (duty cycle based on CPU temp).
Nov 02 16:52:03 storage pi-fan-speed[132437]: CPU Temperature: 42.35°C, Fan Duty Cycle: 30%
```
