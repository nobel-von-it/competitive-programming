package Kata;

import java.util.Arrays;

public class SquareOrPow {
    public static int[] squareOrSquareRoot(int[] a) {
        return Arrays.stream(a).map(e -> {
            if (Math.sqrt(e) == Math.round(Math.sqrt(e)))
                return (int) Math.sqrt(e);
            else return e * e;
        }).toArray();
    }
}
