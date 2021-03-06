# **[`CalcuPi ð`](https://divykj.github.io/CalcuPi/)**

> Calculate Pi using Monte Carlo method

Please â­ if you like it!

## Explanation ð¤

### Steps ð²

- Use a square of arbitrary side length and inscribe a circle inside it.
- Put random points on the square using a uniform distribution.
- Count the number of random points falling inside the circle.
- Calculate the approximate value of pi using, <br />`4 * points inside the circle / total number of points`.

### Maths ð

<details>

<summary>
Why <code>4 * points inside the circle / total number of points</code>?
</summary>

<br />

Let's consider,
<br />
Radius of circle, **`r`**
<br />
Side of square, **`2r`**
<br />
Number of points inside the circle, **`C`**
<br />
Total number of points, **`S`**

Thus,
<br />
Area of circle = **`ÏrÂ²`**
<br />
Area of square = **`4rÂ²`**

For large enough number of random points, we can consider that the ratio of areas of circle to square is equal to the ratio of points inside the circle to the total number of points, ie, **`ÏrÂ²/4rÂ² = C/S`**.
<br />
Which can be simplified to, **`Ï = 4*C/S`**

</details>

## Links â¨

- [What is Monte Carlo Method? ð¤](https://en.wikipedia.org/wiki/Monte_Carlo_method "Monte Carlo method - Wikipedia")
- [Inspiration for the Project ð](https://www.youtube.com/watch?v=5cNnf_7e92Q "Coding Challenge #95: Approximating the Value of Pi - Youtube")
