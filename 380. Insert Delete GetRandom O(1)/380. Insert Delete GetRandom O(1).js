class RandomizedSet {
    constructor() {
      this.valToIndex = new Map(); // Map to store value to index mapping
      this.values = [];            // Array to store the values
    }
  
    insert(val) {
      if (this.valToIndex.has(val)) {
        return false; // Value already exists
      }
      this.valToIndex.set(val, this.values.length);
      this.values.push(val);
      return true;
    }
  
    remove(val) {
      if (!this.valToIndex.has(val)) {
        return false; // Value does not exist
      }
      // Get the index of the value to remove
      const indexToRemove = this.valToIndex.get(val);
      const lastElement = this.values[this.values.length - 1];
  
      // Swap the element to remove with the last element
      this.values[indexToRemove] = lastElement;
      this.valToIndex.set(lastElement, indexToRemove);
  
      // Remove the last element from the list
      this.values.pop();
      this.valToIndex.delete(val);
  
      return true;
    }
  
    getRandom() {
      // Return a random element from the list
      const randomIndex = Math.floor(Math.random() * this.values.length);
      return this.values[randomIndex];
    }
  }
  
  // Example usage:
  const randomizedSet = new RandomizedSet();
  console.log(randomizedSet.insert(1));    // Output: true
  console.log(randomizedSet.remove(2));    // Output: false
  console.log(randomizedSet.insert(2));    // Output: true
  console.log(randomizedSet.getRandom());  // Output: 1 or 2
  console.log(randomizedSet.remove(1));    // Output: true
  console.log(randomizedSet.insert(2));    // Output: false
  console.log(randomizedSet.getRandom());  // Output: 2