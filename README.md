# Spirit

Spirit is an asynchronous I/O framework for Rust with a focus on high performance on modern systems.

Fast programmable NICs and non-volatile main memory (such as [Intel Optane]) are emerging as a foundation for next-generation computer architecture.
This new type of computer architecture challenges the design assumptions of the abstractions and interfaces in our systems today.
For example, kernel-bypass solutions and OS-specific interfaces are replacing POSIX networking and I/O interfaces.

Spirit is a clean-slate interface for implementing high-performance applications.
The design of Spirit is inspired by [Seastar] for async-everywhere and sharding, [Network.framework] for modern networking API, and [Arachne] for light-weight parallelism.

## Features

* Asynchronous interfaces
* Application-level sharding (optional)
* Modern networking API
* Light-weight parallelism

[Arachne]: https://www.usenix.org/system/files/osdi18-qin.pdf
[Intel Optane]: https://www.intel.com/content/www/us/en/architecture-and-technology/intel-optane-technology.html
[Network.framework]: https://developer.apple.com/videos/play/wwdc2018/715/
[Seastar]: http://seastar.io/
