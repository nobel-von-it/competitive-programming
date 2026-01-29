package Kata;

import java.util.stream.IntStream;

public class SumBetwNum {
    public static int GetSum(int a, int b) {
        return IntStream.rangeClosed(Math.min(a, b), Math.max(a, b)).sum();
    }
    // math way
    public static int getSum(int a, int b) {
        return (a + b) * (Math.abs(a - b) + 1) / 2;
    }
}
