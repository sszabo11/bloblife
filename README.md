# Bloblife
## A fun simulation to demonstrate evolution and natural selection

## How it works
# Traits
Each creature currently only has one trait used which is the sense of smell. This value is used to determine the range in which creatures can go for food. The bigger the value, the larger range the creature can "smell" for food.

# Movement
Creatures can move either by moving towards food or in random directions. The distance to all food is calculated then filtered by the range in which their smell allows. If no food is within range then the creature will move in a random direction.

# Energy
Every creature has an energy value which has to be above zero to live. When creatures eat food the energy is incremented. Energy decreased as time goes on. 

# Age
All creatures have age which increments with time. After 50 years old creatures can die at random. Creatures can only reproduce after the age of 20.

# Reproduction
Creatures can reproduce if their energy is above a certain value (40) and they within a certain range of another creature (50) and they are above 20 years old. When children are born the parents' smell is averaged and a random mutation value is added to the smell. This value could be, for example between -0.01 and 0.01. This ensures that new generations have variation that can be selected for.


# Observations
Running the simulation is very satisfying, watching the little creatures run around looking for food. In the status bar at the bottom of the terminal you can see many metrics including the population, average age, average energy and average smell. Watching the average smell go up over time is evidence of natural selection working. Alterting the mutation rate can give you faster change. 

# Next
I would like to build another simulation but with neural networks to simulate brains and "thinking", but I also want to improve this current version. Adding more traits would be cool to see along with more realistic rules. Writing an essay about evolution based on the data would also be rewarding however, I lack the time at this moment. Charting the sense of smell over time and comparing it with the population and other metrics would be fascinating.

# Are we these creatures?
Coming from a science and philosiphical interested background and building this, makes you wonder that we could be the creatures in the simulation created by a God (a teenager in his room). 

I am GOD to these creatures!!!!!.

