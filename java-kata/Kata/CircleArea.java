package Kata;

import java.text.DecimalFormat;

public class CircleArea {
    public static double area(double r) {
        if (r <= 0 )
            throw new IllegalArgumentException();
        return Double.parseDouble(new DecimalFormat("###.###").format(r * r * Math.PI));
    }
}
