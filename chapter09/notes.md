# Chapter 9: Time and timekeeping

Time is not as regular as we'd like, not all the days last the same due to frictions and interferences in the movement
of the Earth, the Moon, etc. Given this lack of regularity, we can find two strategies at dealing with time:

- Keep the length of each second fixed. Drawback: sync with midday is not perfect.
- Adjust the length of the years to keep proper sync with Sun's positioning from year to year.

Atomic clocks follow the first model in a special timezone called TAI. The rest of clocks are usually based on the
second model, following the UTC timezones. UTC adds leap seconds to TAI once every 18 months.

Computers usually have at least two clocks running on the system.

- The **real-time clock**, which is a battery-powered device.
- The **system time**, which increments itself with periodic interruption signals emitted by the system's motherboard.

The source of these interruptions usually is a quartz crystal that oscillates with the electrical stimulation of a
battery. Two counters are updated with this:

- An epochs counter (increments with oscillator ticks).
- A seconds counter (increments for each estimation of a new second).

Devices without a real-time clock (like the Raspberry Pi) connect to get the **epoch time**, which is the count of
elapsed seconds since 1 Jan 1970. Network connection is key for this.

In a running computer, the CPU clock frequency becomes the source of regular ticks. Sometimes, the CPU also has a
register just to keep this count in a dedicated space. Irregularities when one or many CPU are involved also occur.

## Definitions

- **Absolute time** - The time used by humans to communicate between themselves about time. Also known as **calendar
  time**.
- **Real-time clock** - A physical clock embedded in the motherboard which keeps time when the power is off. Also known
  as **CMOS clock**.
- **System clock** - The operating system's view has of the clock. Applications rely on this clock.
- **Monotonically incremented** - A clock that never provides the same timestamp twice (not the case for the system
  clock).
- **High accuracy clock** - Keep seconds of regular length. Little skew (difference) with atomic clocks.
- **Steady clock** - Both monotonically incremented and high accuracy clocks. Bad for keeping absolute time, good for
  counting the duration of a process.
- **High resolution** - Clocks of high accuracy (just nanoseconds). Usually implemented within CPU chips due to their
  speed.
- **Fast clock** - Fast read operations at the cost of worse accuracy and precission.

## Encoding time

Usual approach: two 32 bits integers: one that counts seconds and another one that counts fractions of a second.

- Advantages of having fixed width integers:
  - Simplicity.
  - Efficiency (basic arithmetics).
- Disadvantages:
  - The range is finite.
  - Discrete way of tracking a continuous metric such as time.

Implementations are not consistent across all options. Timezones are also a source of disparity, although a usual
approach is storing another integer with the seconds offset from UTC.

### Time formats

Examples of timestamp formats:

- **Timestamp**, also usually known as *UNIX timestamp*. Number of seconds since the reference epoch.
- **ISO 8601**. A string with information about date, time, timezone, etc.
- **RFC 2822**. Format used by email headers.
- **RFC 3339**. Less strict version of the ISO 8601 standard. A RFC 3339 compliant timestamp is also ISO 8601 compliant,
  but the opposite is not.

## Time in Rust

The community usual choice for time handling in Rust is the `chrono` crate.

Rust compiler refuses to compile a program if finds an illegal arithmetic operation between times (or one that doesn't
take timezones into account).

## Never type

A function that never returns a value is declared as having a `!` return type. The exclamation operation is, then, the
Never type.

```rust
fn setValue(input: number) -> ! {
  something.key = input;
}
```

In this example, the `setValue` function never returns.

## Zero sized types

A struct with no fields is known as a **zero-sized type** or **ZST**.

```rust
struct Clock;
```

ZST structs do not occupy memory in the resulting application, it's a mere compile time construct.

## Command line interface

A popular crate for validating command line inputs in a clean and comfortable manner is `clap`.

Two of the main types `clap` provides are `clap::App` and `clap::Arg`.