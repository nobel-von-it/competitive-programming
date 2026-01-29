package Kata;

import java.util.stream.Stream;

public class VowelChain {
    public static int solve(String s) {
        String v = "aeiou";
        int res = 0, k = 0;
        for (String i: s.split(""))
            if (v.contains(i))
                k++;
            else {
                res = Math.max(res, k);
                k = 0;
            }
        return Math.max(res, k);
    }
    // learn it
    public static int solve2(String s) {
        return Stream.of(s.split("[^aeiou]")).mapToInt(String::length).max().orElse(0);
    }
}
