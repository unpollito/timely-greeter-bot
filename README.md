# timely-greeter-bot

This is a Telegram bot that greets users at 9 am.

9 am, which timezone? All of them.

So at 9 am UTC it send something like "Good morning Canary Islands, Lisbon and London" if DST is not enabled, or "Good morning Azores" if it is. At 8 am UTC it will send "Good morning Berlin, Paris, and Madrid" if DST is not active, or "Good morning Canary Islands, Lisbon and London" if it is. And that for almost every timezone that there is in the world.

It's not meant to be very useful, just a way to annoy a friend who complained that I'd greet them with a "good morning" sticker in the evening.

## Setup

Clone and create an `.env` file:

```
TELEGRAM_BOT_TOKEN=<your token here>
```

Then `cargo run` normally, or `cargo build`, whatever floats your boat.

## Usage

- `/start` (or anything else, really) signs up a user to start receiving greetings.
- `/stop` unsubscribes a user.
