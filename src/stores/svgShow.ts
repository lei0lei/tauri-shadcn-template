import { create } from "zustand";

type HolePosition = {
  artifact: string;     // 型号，如 "EH09"
  surface: string;      // 面，如 "A"
  holeId: number;       // 孔编号
  x: number;            // 在 SVG 中的横坐标
  y: number;            // 在 SVG 中的纵坐标
  r: number;            //直径
};

interface HolePositionStore {
  holePositions: HolePosition[]; // 所有位置数据
  getHolePositions: (artifact: string, surface: string, holeId:number) => HolePosition[];
}

export const useHoleStore = create<HolePositionStore>(() => {
  const holePositions: HolePosition[] = [
    // EH09 - A面
    // { artifact: "EH09", surface: "A", holeId: 1, x: 100, y: 150 },
    // { artifact: "EH09", surface: "A", holeId: 2, x: 200, y: 250 },
    // { artifact: "EH09", surface: "A", holeId: 3, x: 300, y: 350 },

    // EH09 - B面
    // { artifact: "EH09", surface: "B", holeId: 1, x: 120, y: 160 },
    // { artifact: "EH09", surface: "B", holeId: 2, x: 220, y: 260 },

    // EH09 - C面

    // EH09 - D面

    // EH09 - E面

    // EH09 - F面

    // EH12 - A面
    { artifact: "EH12", surface: "A", holeId: 1, x: 364, y: 228 ,r:15},
    { artifact: "EH12", surface: "A", holeId: 2, x: 504, y: 470 ,r:15},
    { artifact: "EH12", surface: "A", holeId: 3, x: 456, y: 670 ,r:20},
    { artifact: "EH12", surface: "A", holeId: 4, x: 316, y: 474 ,r:15},
    // EH12 - B面
    { artifact: "EH12", surface: "B", holeId: 1, x: 152, y: 235 ,r:9},
    { artifact: "EH12", surface: "B", holeId: 2, x: 220, y: 265 ,r:13},
    { artifact: "EH12", surface: "B", holeId: 3, x: 332, y: 195 ,r:13},
    { artifact: "EH12", surface: "B", holeId: 4, x: 505, y: 204 ,r:13},
    { artifact: "EH12", surface: "B", holeId: 5, x: 661, y: 238 ,r:9},
    { artifact: "EH12", surface: "B", holeId: 6, x: 427, y: 320 ,r:9},
    { artifact: "EH12", surface: "B", holeId: 7, x: 493, y: 362 ,r:12},
    { artifact: "EH12", surface: "B", holeId: 8, x: 487, y: 526 ,r:12},
    { artifact: "EH12", surface: "B", holeId: 9, x: 570, y: 655 ,r:9},
    { artifact: "EH12", surface: "B", holeId: 10, x: 222, y: 645 ,r:7},
    { artifact: "EH12", surface: "B", holeId: 11, x: 322, y: 524 ,r:12},
    { artifact: "EH12", surface: "B", holeId: 12, x: 157, y: 388 ,r:12},
    { artifact: "EH12", surface: "B", holeId: 13, x: 318, y: 360 ,r:12},
    // EH12 - C面
    { artifact: "EH12", surface: "C", holeId: 1, x: 308, y: 215 ,r:9},
    { artifact: "EH12", surface: "C", holeId: 2, x: 476, y: 124 ,r:9},
    { artifact: "EH12", surface: "C", holeId: 3, x: 452, y: 326 ,r:12},
    { artifact: "EH12", surface: "C", holeId: 4, x: 464, y: 397 ,r:9},
    { artifact: "EH12", surface: "C", holeId: 5, x: 396, y: 649 ,r:9},
    { artifact: "EH12", surface: "C", holeId: 6, x: 328, y: 656 ,r:20},
    // EH12 - D面
    { artifact: "EH12", surface: "D", holeId: 1, x: 234, y: 283 ,r:9},
    { artifact: "EH12", surface: "D", holeId: 2, x: 273, y: 166 ,r:10},
    { artifact: "EH12", surface: "D", holeId: 3, x: 418, y: 282 ,r:9},
    { artifact: "EH12", surface: "D", holeId: 4, x: 582, y: 296 ,r:9},
    { artifact: "EH12", surface: "D", holeId: 5, x: 611, y: 492 ,r:9},
    { artifact: "EH12", surface: "D", holeId: 6, x: 512, y: 660 ,r:9},
    { artifact: "EH12", surface: "D", holeId: 7, x: 376, y: 656 ,r:9},
    { artifact: "EH12", surface: "D", holeId: 8, x: 229, y: 656 ,r:9},
    { artifact: "EH12", surface: "D", holeId: 9, x: 174, y: 479 ,r:9},
    // EH12 - E面
    { artifact: "EH12", surface: "E", holeId: 1, x: 530, y: 230 ,r:17},
    { artifact: "EH12", surface: "E", holeId: 2, x: 549, y: 533 ,r:18},
    { artifact: "EH12", surface: "E", holeId: 3, x: 190, y: 520 ,r:18},
    { artifact: "EH12", surface: "E", holeId: 4, x: 208, y: 240 ,r:18},
    // EY28 - A面
    { artifact: "EY28", surface: "A", holeId: 1, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "A", holeId: 2, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "A", holeId: 3, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "A", holeId: 4, x: 530, y: 230 ,r:17},
    // EY28 - B面
    { artifact: "EY28", surface: "B", holeId: 1, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "B", holeId: 2, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "B", holeId: 3, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "B", holeId: 4, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "B", holeId: 5, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "B", holeId: 6, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "B", holeId: 7, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "B", holeId: 8, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "B", holeId: 9, x: 530, y: 230 ,r:17},
    { artifact: "EY28", surface: "B", holeId: 10, x: 530, y: 230 ,r:17},
    // EY28 - C面

    // EY28 - D面

    // EY28 - E面

    // EK30 - A面

    // EK30 - B面

    // EK30 - C面

    // EK30 - D面

    // EK30 - E面

    // EK40 - A面

    // EK40 - B面

    // EK40 - C面

    // EK40 - D面

    // EK40 - E面


  ];

  return {
    holePositions,
    getHolePositions: (artifact, surface, holeId) =>
      holePositions.filter((pos) => pos.artifact === artifact && pos.surface === surface && pos.holeId == holeId),
  };
});