# Zing
Zing is a lightweight tool that lets you play music using your motherboard's internal POST speaker (buzzer). Unlike traditional sound cards, Zing directly controls the system buzzer to generate simple tones and melodies. Perfect for nostalgic beeps, chiptune-style music, or system notifications!

## Features
- Play songs through the POST buzzer (internal PC speaker)
- Supports custom melodies and note sequences from files
- Runs as a daemon, allowing playback from user commands
- Ideal for system alerts, notifications, and fun projects
- Works on Linux with root privileges or via a background service

## Installation (Arch Linux)
Zing comes in 2 parts: the CLI and the daemon. You will require [the rust toolchain](https://www.rust-lang.org/learn/get-started) to compile either of these.

Oh, you will also need a buzzer on your board. Can't forget the important bit!

### Building
To build zing, clone the repository and run cargo install:

```sh
git clone https://github.com/Daxanius/zing.git
cd zing
sudo cargo install --path sys-zing-daemon/ --root /usr
sudo cargo install --path zing-cli/ --root /usr
```

You have now built both zing binaries. However, we still need to setup the zing system service.

### Systemd Service
Since I use systemd myself, I will simply explain how you can set up the service using systemd. For any other init systems, you are on your own.

Create the following file with nano:
```sh
sudo nano /etc/systemd/system/syszingd.service
```

Then paste in the following configuration, feel free to modify it to your liking:
```ini
[Unit]
Description=System Zing Daemon
After=network.target

[Service]
Environment="RUST_LOG=info"
ExecStart=/usr/bin/syszingd
WorkingDirectory=/usr/bin
SyslogIdentifier=syszingd
Restart=always
User=root
Group=root

[Install]
WantedBy=multi-user.target
```

After you've saved your service file, start the service:
```sh
sudo systemctl daemon-reload && sudo systemctl enable --now syszingd.service
```

You are now ready to use the CLI as a non-root user!

## Usage
As of writing this, the zing CLI allows you to play, stop, pause and resume melodies. To view the possible options, run:
```sh
zing help
```

It will provide you with everything necessary for use.

## Notemap Files (.nm)
Since playing waveform audio with beeps and boops isn't feasible, I decided to opt for a simpler, more straight-forward approach. However this does not mean I'm not open to implementing more complex formats such as [MIDI files](https://en.wikipedia.org/wiki/MIDI).

I lazily labelled this file format notemap (.nm). It might seem a little intimidating at first, but the format is very simple in practice! 

### Notemap Format
Notemaps are essentially a straight copy-paste from https://pianoletternotes.blogspot.com/. Meaning they are text-files. They describe their own format really well, so I will just quote them on this with slight tweaks for zing:

> Each group of letter notes is played from left to right, and vertical letters on the same column are played together. The numbers in front of each line are the octave.
> 
> Lowercase (a b c d e f g) letters are natural notes (white keys, a.k.a A B C D E F G ). Uppercase (A C D F G) letters are the sharp notes (black keys a.k.a. A# C# D# F# G#).
> 
> The lines / dashes (-) between letters indicates timing to play the notes. (usually 5-6 dashes is about 1 second)
> 
> RH / LH means Right Hand / Left Hand. This is ignored by zing.

To provide an example of this format in action:

![Notemap Format Exampel Gif](https://lh3.googleusercontent.com/-YcLZY4pkdL0/WwWuPoSOYkI/AAAAAAACdro/ykwWpSSUHIc9mdTaIQGVbnQtY3LGo2_VwCLcBGAs/h200/how%2Bto%2Bread%2Band%2Bplay%2Bthe%2Bletter%2Bnotes.gif)

### Zing Tweaks
Since the format was designed for humans to learn playing the piano, not for programs to interpret willy nilly, I had to establish some basic rules for zing. Any line containing exactly 2 pipes (|) will be interpreted as a zing line, any other lines are considered comments. If you want to comment out a valid line, you can prefix it with '#'. It is recommended to do this for every comment.
```
# This is a comment and will be ignored by zing :)
5|--e----e------e---e----e--|
4|eg-ebag-abegbg-eeg-ebag-ab|

# This line was a failed attempt at making a melody :(
# 4|eg-ebag-abegbg-eeg-ebag-ab|

5|g---E---E----e------e---e-|
4|egbg-eeg-ebag-abegbg-eeg-e|
```

## Limitations
Due to both buzzer and system limitations, zing had to come up with some solutions in order to provide a good experience playing sounds on the buzzer.

### Linux (Unix) Only
Unfortunately as to my knowledge, Windows does not provide access to a buzzer API. Due to this, I made it so that this project is Linux only.

### Daemon
You may have noticed that this project has a daemon program included (sys-zing-daemon or syszingd for short). This is for a very good reason: controlling the buzzer requires system privileges. Not only that, but imagine if a million programs were trying to use the single poor buzzer at the same time?

To solve this, I created a daemon service which zing communicates with in order to play melodies. If another service were to use the same service, the service will take care of it gracefully. 

However, I do not know what would happen if another root process were to access the buzzer directly. Use it at your own risk.

### Melodies
As you might have guessed, since it uses your buzzer, the melodies you can play are limited to chirps and beeps. Maybe you'll like that, maybe you wont. Either way, zing tries its best to make your melodies sound as crisp as possible!

Since the buzzer can only play a single tune (frequency) at a time, zing tries to get around this for Chords by iterating through the frequencies rapidly.

## Contributing
I am open to review pull-requests and suggestions/ideas. The project is nowhere near a finished or perfect state, and there are many things that I would still like to add/change. However, I cannot guarantee that I will remain active on a regular basis, for I am a busy person.

For this reason, anyone interested in contributing is welcome, however I do recommend you to follow the code style, unless you have a better alternative and can provide a good reason for switching to said alternative.