package Kata;

import java.util.Arrays;

public class ShortWord {
    public static int findShort(String s) {
        return Arrays.stream(s.split(" ")).mapToInt(String::length).min().getAsInt();
    }
    public static int findShortFast(String s) {
        String[] a = s.split(" ");
        int min = 1000;
        for (String el : a)
            if (el.length() < min)
                min = el.length();
        return min;
    }
}
