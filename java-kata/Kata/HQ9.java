package Kata;

public class HQ9 {
    public static String HQ9(char code) {
        String str = "";
        switch (code) {
            case 'H' -> str = "Hello World!";
            case 'Q' -> str = "Q";
            case '9' -> {
                for (int i = 99; i >= 0; i--) {
                    switch (i) {
                        case 2 -> str += """
                                2 bottles of beer on the wall, 2 bottles of beer.
                                Take one down and pass it around, 1 bottle of beer on the wall.
                                """;
                        case 1 -> str += """
                                1 bottle of beer on the wall, 1 bottle of beer.
                                Take one down and pass it around, no more bottles of beer on the wall.
                                """;
                        case 0 -> str += """
                                No more bottles of beer on the wall, no more bottles of beer.
                                Go to the store and buy some more, 99 bottles of beer on the wall""";
                        default -> str += String.format("""
                                %d bottles of beer on the wall, %d bottles of beer.
                                Take one down and pass it around, %d bottles of beer on the wall.
                                """, i, i, i-1);
                    }
                }
            }
            default -> {
                return null;
            }
        }
        return str;
    }
}
