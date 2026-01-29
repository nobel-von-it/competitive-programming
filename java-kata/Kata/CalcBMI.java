package Kata;

public class CalcBMI {
    public static String bmi(double w, double h) {
        double bmi = w / (h * h);
        if (bmi > 30)
            return "Obese";
        else if (bmi <= 18.5)
            return "Underweight";
        else if (bmi <= 25.0)
            return "Normal";
        else return "Overweight";
    }
}
