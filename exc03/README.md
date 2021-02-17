# Task 03

Example solution of console car sensors simulation. I have focused on creating reusable code, 
one that can be easily extended.

Module `view` contains Logger printing periodically status of each sensor declared 
in `main` function. Inside `model` can be found declaration of every sensor used. 



Example output:
```
Time 1.0010582 | engine: rpm=2500 temp=81
Time 2.002117  | engine: rpm=2503 temp=84
Time 3.0031755 | engine: rpm=2503 temp=84 | wheel_angle=0.16615523
Time 4.004234  | engine: rpm=2503 temp=84 | wheel_angle=24.378548 | vehicle_acceleration=(-5, 2, 1); vehicle_speed=12.072305 | tires_pressure=(2.15, 2.15, 2.15, 2.16)
Time 5.005292  | engine: rpm=2488 temp=69
Time 6.0063505 | engine: rpm=2498 temp=79
Time 7.0074096 | engine: rpm=2496 temp=77 | wheel_angle=0.36965215
Time 8.008469  | engine: rpm=2509 temp=90 | wheel_angle=5.3609314 | vehicle_acceleration=(-6, 2, 1); vehicle_speed=15.88048 | tires_pressure=(2.15, 2.15, 2.15, 2.16)
Time 9.009525  | engine: rpm=2511 temp=92
Time 10.010584 | engine: rpm=2521 temp=102
Time 11.011642 | engine: rpm=2511 temp=92 | wheel_angle=0.33313584
Time 12.012701 | engine: rpm=2511 temp=92 | wheel_angle=2.3068268 | vehicle_acceleration=(-11, -1, -1); vehicle_speed=7.2211103 | tires_pressure=(2.15, 2.15, 2.15, 2.16)
Time 13.013759 | engine: rpm=2506 temp=87
Time 14.014817 | engine: rpm=2503 temp=84
```

## Running
Type command `cargo run`