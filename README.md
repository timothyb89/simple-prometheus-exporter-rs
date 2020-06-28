# simple-prometheus-exporter

A trivial Prometheus exporter library and helper macro for generating
Prometheus-compatible metric endpoints. You should use the regular
[rust-prometheus crate](https://github.com/tikv/rust-prometheus) for serious
production use.

## Use cases

This library is well suited for converting other APIs or sensors to the
Prometheus metric format, particularly where fields exposed by the external
source vary over time. It's often better to have gaps in the time series than to
back-fill the last value, however traditional exporter libraries make this
difficult.

Instead, this library helps applications generate the metrics response from
scratch, making it easy to conditionally format the response based on whatever
metrics are actually available and accurate at a particular instant.

## Examples

These projects make use of this exporter library:

 * https://github.com/timothyb89/co2-exporter - an exporter for USB CO2 sensors
 * https://github.com/timothyb89/openweathermap-exporter - an exporter for
   OpenWeatherMap

## Limitations and notes

 * This library only formats metrics; mapping this to an HTTP endpoint
   is left as an exercise to the reader.
 * This library is compatible with Prometheus's [exposition format][exposition]
   but does not enforce its rules. Users are responsible for using a "supported
   metric primitive", at least to the extent they're more than key/value pairs.
 * No prometheus metric or label descriptions are written. The `HELP` and `TYPE`
   fields aren't actually required (or to my knowledge, used at all, except by
   human readers) and are not supported.
 * Users are responsible for following (or not following) the
   [metric naming conventions][naming]
 * It's not particularly efficient and probably clones strings too much

[exposition]: https://github.com/prometheus/docs/blob/master/content/docs/instrumenting/exposition_formats.md#text-based-format
[naming]: https://prometheus.io/docs/practices/naming/
