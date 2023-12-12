using System;
using System.Collections.Generic;

namespace QuadraticEqRoot;

internal class QuadraticEquationSolver
{
    private const double Eps = 0.00000001;

    private static (double, double, double) ParseCoefficients()
    {
        Console.WriteLine("Please set the quadratic equation (ax^2 + bx + c = 0)");

        var coefficients = new Dictionary<char, double>
        {
            {'a', 0},
            {'b', 0},
            {'c', 0}
        };

        foreach (var coefficientName in coefficients.Keys)
        {
            double coefficientValue;
            do
            {
                Console.WriteLine(
                    "Please type a correct value for the '{0}' coefficient",
                    coefficientName
                );
            } while (!double.TryParse(Console.ReadLine(), out coefficientValue));

            coefficients[coefficientName] = coefficientValue;
        }

        return (coefficients['a'], coefficients['b'], coefficients['c']);
    }

    private static double GetDiscriminant(double a, double b, double c)
    {
        return b * b - 4 * a * c;
    }

    private static (double, double)? GetRootsOfQuadraticEquation(double a, double b, double c)
    {
        var discriminant = GetDiscriminant(a, b, c);
        if (discriminant < 0) return null;

        if (Math.Abs(discriminant) < Eps)
        {
            var root = -b / (2 * a);
            return (root, root);
        }

        var sqrtDiscriminant = Math.Sqrt(discriminant);
        var firstRoot = (-b + sqrtDiscriminant) / (2 * a);
        var secondRoot = (-b - sqrtDiscriminant) / (2 * a);
        return (firstRoot, secondRoot);
    }

    private static void PrintRoots((double, double)? root)
    {
        if (!root.HasValue)
        {
            Console.WriteLine("There are no real roots");
            return;
        }

        var rootDifference = Math.Abs(root.Value.Item1 - root.Value.Item2);
        if (rootDifference < Eps)
            Console.WriteLine("There is one root: {0}", root.Value.Item1);
        else
            Console.WriteLine("There are two roots: {0} and {1}", root.Value.Item1, root.Value.Item2);
    }

    private static bool CheckEdgeCase(double a, double b, double c)
    {
        if (!(Math.Abs(a) < Eps)) return false;

        if (Math.Abs(b) < Eps)
        {
            Console.WriteLine(Math.Abs(c) < Eps ? "X can be any value" : "There are no real roots");
        }
        else
        {
            var root = -c / b;
            Console.WriteLine("There is one root: {0}", root);
        }

        return true;
    }

    public static void Main(string[] args)
    {
        var (a, b, c) = ParseCoefficients();
        var isEdgeCase = CheckEdgeCase(a, b, c);
        if (!isEdgeCase)
        {
            var roots = GetRootsOfQuadraticEquation(a, b, c);
            PrintRoots(roots);
        }
    }
}