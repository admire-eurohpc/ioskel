# IOSKEL
A simple skeleton application for I/O testing

## Overview
IOSKEL allows you to create parameterized I/O patterns in READ and WRITE over time. It can be used to test an I/O monitoring stack.

## Usage
The main way to use ioskel is by running the `run.sh` script, which provides an example of how to use it. The principle is that a single binary is launched with various parameters, using the following template:

`ioskel.${comp}.${size}.${iter}.${proba}`

Where:
- `comp`: duration of compute time
- `size`: size to write in MB
- `iter`: number of iterations
- `proba`: probability of transitioning from WRITE to READ (default is WRITE)

You can create symbolic links to a single binary to create multiple application behaviors while maintaining a different name for bookkeeping.

## Command-line Options

```
Usage: ioskel [OPTIONS]

Options:
  -c, --compute-time <COMPUTE_TIME>  compute time of the IO start
  -s, --size <SIZE>                Size of IO
  -i, --iter <ITER>                Iterations of IO
  -t, --transition-probability <TRANSITION_PROBABILITY>
  -h, --help                      Print help
  -V, --version                   Print version
```

## Acknowledgments

This project has received funding from the European Union’s Horizon 2020 JTI-EuroHPC research and innovation programme with grant Agreement number: 956748


