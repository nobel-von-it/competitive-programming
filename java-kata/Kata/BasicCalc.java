package Kata;

public class BasicCalc {
    public static Integer basicMath(String op, int a, int b) {
        return switch (op) {
            case "+" -> a + b;
            case "-" -> a - b;
            case "/" -> a / b;
            case "*" -> a * b;
            default -> throw new IllegalArgumentException("aaaa");
        };
    }
}
