package Kata;

import java.util.stream.IntStream;

public class CalcAverage {
    public static double find_average(int[] a) {
        return IntStream.of(a).sum() / (double)(a.length);
    }
    public static double average(int[] a) {
        return IntStream.of(a).average().orElse(0);
    }
}
