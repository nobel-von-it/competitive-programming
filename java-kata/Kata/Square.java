package Kata;

public class Square {
    public static boolean isSquare(int n) {
        return Math.sqrt(n) == Math.round(Math.sqrt(n));
    }
}
