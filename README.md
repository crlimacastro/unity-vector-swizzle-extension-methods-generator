# What does it do?

It generates a file with C# extension methods for Unity's ```Vector2```, ```Vector2Int```, ```Vector3```, & ```Vector3Int``` classes for 
[swizzling](https://en.wikipedia.org/wiki/Swizzling_(computer_graphics)) (TL;DR re-arranging) their fields easily.

Copy into your project to start using them.

# Usage

```cs
Vector3 v = new Vector3(1f, 2f, 3f);

// You can re-arrange the components of a vector in any order by swizzling
Vector3 vBackwards = v._zyx; // will be equal to [3f, 2f, 1f]
Vector3 vShuffled = v._yxz; // will be equal to [2f, 1f, 3f]

// You can set any individual component to 0 or 1
Vector3 vWithYZero = v._x0z; // will be equal to [1f, 0f, 3f]
Vector3 vWithZOne = v._xy1; // will be equal to [1f, 0f, 1f]

// You can repeat component values
Vector3 vWithXRepeatedOnce = v._xxz; // will be equal to [1f, 1f, 3f]
Vector3 vWithXRepeatedAll = v._xxx; // will be equal to [1f, 1f, 1f]

// You can set multiple components to 0 or 1
Vector3 vWithOneZeroZero = v._100; // will be equal to [1f, 0f, 0f] (equivalent to Vector3.right)

// You can set individual or multiple components to a specific value 'p'
Vector3 vWithXFive = v._pyz(5f); // will be equal to [5f, 2f, 3f]
Vector3 vWithYZFive = v._xpp(5f); // will be equal to [1f, 5f, 5f]
```

# Generator Instructions
Make sure you have [Rust](https://www.rust-lang.org/) installed.

Open your terminal to the root of the project and run

```
cargo run
```

Your file will be in ```out/VectorSwizzleExtensions.cs```

# ⚠ If you are only interested in the swizzle extensions & not the generator

The file is in ```out/VectorSwizzleExtensions.cs``` from the root of the project