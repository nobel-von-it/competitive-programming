package Kata;

public class WhoLikes {
    public static String whoLikesIt(String... names) {
        String res = " like this";
        switch (names.length) {
            case 0 -> res = "no one likes this";
            case 1 -> res = names[0] + " likes this";
            case 2 -> res = names[0] + " and " + names[1] + res;
            case 3 -> res = names[0] + ", " + names[1] + " and " + names[2] + res;
            default -> res = names[0] + ", " + names[1] + " and " + (names.length-2) + " others" + res;
        }
        return res;
    }
}
