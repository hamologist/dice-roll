# Dice Roll
**Dice Roll** is a Rust project for simulating dice rolls with varying sides and counts.
The package includes both an API and CLI frontend for running the dice roll logic.

## Installation

### Local Installation
You can install the package directly to your machine using cargo:
```bash
$ cargo install --git https://github.com/hamologist/dice-roll.git --branch main
```

Likewise, you can uninstall the application using:
```bash
$ cargo uninstall dice-roll
```

### Docker
You can also install and run the package using Docker like so:

First, build the image for the package:
```bash
$ docker build -t dice-roll https://github.com/hamologist/dice-roll.git#main
```

Next, run a container using the image you built:
```bash
$ docker run -p 3000:3000 --rm dice-roll
```
This will run the `dice-roll-api` command (further detailed below) on host 0.0.0.0 and port 3000.

If you'd rather run `dice-roll` via the CLI, you can do so using:
```bash
$ docker run --rm -it dice-roll /bin/sh
```
This will connect you to an interactive shell on the `dice-roll` container.
You can then run `dice-roll` using the following:
```bash
$ echo '1d20 + 2d4 + 1' | dice-roll
```

## Usage
Once the dice-roll frontends have been installed, you can start interfacing with both.
### CLI
A `dice-roll` command will be intalled on your system.
Help can be pulled up using the help flag:
```bash
$ dice-roll --help
```

The command takes a dice roll instruction using text sent via STDIN or using a file on your local machine.
Here is an example of what using the tool looks like:
```bash
$ echo '1d20 + 1d4 + 2' | dice-roll
```
The above will return a human readable breakdown of the resulting roll like so:
```bash
(4 of 20) + (3 of 4) + 2 = 9
```

You can opt to have the CLI return the roll in a JSON format as well using the `--as-json` flag.
```bash
$ echo '1d20 + 1d4 + 2' | dice-roll --as-json
```
That should return something that looks like the following:
```bash
{
  "rolls": [
    {
      "count": 1,
      "modifier": 0,
      "rolls": [
        14
      ],
      "sides": 20,
      "total": 14
    },
    {
      "count": 1,
      "modifier": 2,
      "rolls": [
        3
      ],
      "sides": 4,
      "total": 5
    }
  ],
  "total": 19
}
```
### API
A `dice-roll-api` command will be installed on your system.
If executed, the server will start running on host 0.0.0.0 and port 3000 by default.
These values can be changed using the `--host` and `--port` command line arguments.
The server takes requests on its "/" endpoint. Requests must be a POST.

The endpoint's accepted payload uses the following structure:
```json
{
    "dice": [
        {
            "count": {num-of-dice-with-provided-side-count-to-roll},
            "sides": {num-of-how-many-sides-current-dice-instance-should-have},
            "modifier": {optional-num-for-what-modifier-should-be-applied-to-dice-instance}
        }
    ],
}
```

You can hit the server using curl like this:
```bash
curl --location --request POST 'localhost:3000' \
--header 'Content-Type: application/json' \
--data-raw '{
    "dice": [
        {
            "count": 1,
            "sides": 20,
            "modifier": 1
        },
        {
            "count": 1,
            "sides": 4,
            "modifier": 2
        }
    ]
}'
```
