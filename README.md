# Three Body Problem Simulation

## Step 1: Project Setup

Project Directory: run the command `cargo new three_body_simulation`

## Step 2: Building a Visualizer and Input Handler

A. Define Visualization Requirements
What to Display: Decide on the information to visualize (e.g., positions, velocities, masses, and gravitational forces of the bodies).
How to Display: Consider the mode of visualization (2D or 3D), colors, scales for forces, and trajectories.

B. Choose a Visualization Library
These libraries allow for creating graphical windows, drawing shapes, and handling user input. Chose Nannou.

C. Input Handling
Starting Conditions: Allow users to input initial positions, velocities, and masses of the bodies either through command line arguments, a configuration file, or an interactive graphical interface.
Real-time Interaction: Consider mechanisms for users to modify parameters in real-time (e.g., adjusting mass of a body) and observe how the system evolves in response.

D. Implementing the Visualizer
Setup: Initialize the chosen graphics library in your project. For libraries like SDL or OpenGL, this involves creating a window and setting up a render loop.
Drawing Functions: Implement functions to draw the bodies and possibly their trajectories. You might use simple shapes (circles, spheres) to represent the bodies.
Real-time Data Update: Ensure the visualizer can fetch and display updated data at each simulation step. This includes recalculating positions and

## Step 3: Design Simulation

Mathematical Model: Define the mathematical model for the gravitational forces and motions. Newton's laws of motion and universal gravitation are key.
Data Structures: Design structures to represent celestial bodies (e.g., position, velocity, mass).
Initialization: Code to initialize the system with starting positions, velocities, and masses.

## Step 4: Implement Core Functionality

Force Calculation: Implement a function to calculate the gravitational force between any two bodies.
Motion Update: Update the positions and velocities of the bodies over time using the forces calculated.
Time Step Control: Implement logic to progress the simulation through small time steps.

## Step 5: Simulation Loop

Main Loop: Create a loop in main.c to run the simulation for a specified duration, updating and logging the state of the system at each step.

## Step 6: Output and Visualization

Logging: Implement logging of positions and velocities to files or the console for analysis.
Visualization (Optional): Consider tools or libraries for visualizing the motion of the bodies over time.

## Step 7: Testing and Debugging

Unit Tests: Write tests for key functions, especially calculations.
Debugging: Use debugging tools to identify and fix issues.

## Step 8: Optimization (Optional)

Performance Analysis: Identify bottlenecks using profiling tools.
Optimization: Optimize calculations, possibly using parallel processing techniques.