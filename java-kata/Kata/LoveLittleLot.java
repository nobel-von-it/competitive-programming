package Kata;

public class LoveLittleLot {
    public static String howMuchILoveYou(int n) {
        return switch (n % 6) {
            case 1 -> "I love you";
            case 2 -> "a little";
            case 3 -> "a lot";
            case 4 -> "passionately";
            case 5 -> "madly";
            case 0 -> "not at all";
            default -> throw new IllegalStateException("Unexpected value: " + n % 6);
        };
    }
}
