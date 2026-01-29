package Kata;

import java.util.stream.IntStream;

public class GrassHopper {
    public static int summation(int n) {
        return IntStream.range(1, n+1).sum();
    }
}
