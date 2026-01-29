package Kata;

public class Disemvowel {
    public static String disemvowel(String s) {
        return s.replaceAll("(?i)[aeiou]", "");
    }
}
