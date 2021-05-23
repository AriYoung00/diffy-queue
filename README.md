# diffy-queue
Another Rust library to computationally solve ODEs.

Thus far, have implemented Euler's method and Runge-Kutta 4th order. Here is an example graph for dx/dt = tx comparing
the output of Euler's method and RK4 with the actual solution to this differential equation, x(t) = e^(0.5 * t^2):

!(Example Graph)[images/test_chart.png?raw=true]