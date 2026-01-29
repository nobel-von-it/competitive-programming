package Kata;

public class Maskify {
    public static String maskify(String s) {
        return s.length() > 4 ? "#".repeat(s.length()-4) + s.substring(s.length()-4) : s;
    }
}
