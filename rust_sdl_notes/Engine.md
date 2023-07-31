This is an object that contains everything for our engine. This mostly only exists because we need to initialize SDL2.

The rendering processes and updating processes will run on their own threads. The rendering processes will be on the main thread for now and the updating processes on a new one.