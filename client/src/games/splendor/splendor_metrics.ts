import { Card, ColoredGemCounts } from "./splendor_rules";

export function totalGemCounts(cards: Array<Card>): ColoredGemCounts {
  const total: ColoredGemCounts = {
    purple: 0,
    red: 0,
    orange: 0,
    blue: 0,
    yellow: 0,
  };

  cards.forEach((card) => {
    total.purple += card.cost.purple;
    total.red += card.cost.red;
    total.orange += card.cost.orange;
    total.blue += card.cost.blue;
    total.yellow += card.cost.yellow;
  });

  return total;
}
