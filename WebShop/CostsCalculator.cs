public static class CostsCalculator {
  public static double ShippingCosts(bool calculateShippingCosts, string typeOfShippingCosts, double totalPrice) {
    double result = 0;
    if (calculateShippingCosts == true) {
      if (totalPrice > 1500) {
        result = 0;
      } else {
        switch (typeOfShippingCosts) {
        case "Ground":
          result = 100;
          break;
        case "InStore":
          result = 50;
          break;
        case "NextDayAir":
          result = 250;
          break;
        case "SecondDayAir":
          result = 125;
          break;
        default:
          result = 0;
          break;
        }
      }
    } else {
      result = 0;
    }
    return result;
  }
}