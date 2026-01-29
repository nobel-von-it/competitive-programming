package Kata;

import java.util.Arrays;

public class NumToRevArr {
    public static int[] digitize(long n) {
        return Arrays.stream(new StringBuilder(String.valueOf(n))
                .reverse()
                .toString()
                .split(""))
                .mapToInt(Integer::parseInt)
                .toArray();
    }
    //or simple
    public static int[] digitize2(long n) {
        return new StringBuilder().append(n)
                .reverse()
                .chars()
                .map(Character::getNumericValue)
                .toArray();
    }
}
