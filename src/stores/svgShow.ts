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
    { artifact: "EH09", surface: "A", holeId: 1, x: 328, y: 458 ,r:12},
    { artifact: "EH09", surface: "A", holeId: 2, x: 483, y: 455 ,r:15},

    // EH09 - B面
    { artifact: "EH09", surface: "B", holeId: 1, x: 558, y: 625 ,r:15},
    { artifact: "EH09", surface: "B", holeId: 2, x: 449, y: 619 ,r:9},
    { artifact: "EH09", surface: "B", holeId: 3, x: 296, y: 611 ,r:9},
    { artifact: "EH09", surface: "B", holeId: 4, x: 223, y: 580 ,r:11},
    { artifact: "EH09", surface: "B", holeId: 5, x: 240, y: 515 ,r:9},
    { artifact: "EH09", surface: "B", holeId: 6, x: 186, y: 200 ,r:15},
    { artifact: "EH09", surface: "B", holeId: 7, x: 312, y: 222 ,r:9},
    { artifact: "EH09", surface: "B", holeId: 8, x: 581, y: 184 ,r:11},
    { artifact: "EH09", surface: "B", holeId: 9, x: 554, y: 475 ,r:9},
  

    // EH09 - C面
    { artifact: "EH09", surface: "C", holeId: 1, x: 490, y: 575 ,r:9},
    { artifact: "EH09", surface: "C", holeId: 2, x: 320, y: 531 ,r:9},
    { artifact: "EH09", surface: "C", holeId: 3, x: 310, y: 292 ,r:12},
    { artifact: "EH09", surface: "C", holeId: 4, x: 452, y: 398 ,r:12},
    { artifact: "EH09", surface: "C", holeId: 5, x: 404, y: 467 ,r:9},

  
    // EH09 - D面
    { artifact: "EH09", surface: "D", holeId: 1, x: 606, y: 479 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 2, x: 500, y: 539 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 3, x: 448, y: 623 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 4, x: 345, y: 539 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 5, x: 246, y: 521 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 6, x: 352, y: 400 ,r:12},
    { artifact: "EH09", surface: "D", holeId: 7, x: 204, y: 370 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 8, x: 200, y: 225 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 9, x: 260, y: 150 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 10, x: 376, y: 136 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 11, x: 520, y: 146 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 12, x: 611, y: 220 ,r:9},
    { artifact: "EH09", surface: "D", holeId: 13, x: 607, y: 352 ,r:9},
    // EH09 - E面
    { artifact: "EH09", surface: "E", holeId: 1, x: 492, y: 640 ,r:20},
    { artifact: "EH09", surface: "E", holeId: 2, x: 400, y: 630 ,r:20},
    { artifact: "EH09", surface: "E", holeId: 3, x: 245, y: 654 ,r:20},
    { artifact: "EH09", surface: "E", holeId: 4, x: 246, y: 179 ,r:20},
    { artifact: "EH09", surface: "E", holeId: 5, x: 394, y: 157 ,r:20},
    { artifact: "EH09", surface: "E", holeId: 6, x: 490, y: 158 ,r:20},
    // EH09 - F面
    { artifact: "EH09", surface: "F", holeId: 1, x: 508, y: 572 ,r:24},
    { artifact: "EH09", surface: "F", holeId: 2, x: 552, y: 212 ,r:26},
    { artifact: "EH09", surface: "F", holeId: 3, x: 208, y: 194 ,r:26},
    { artifact: "EH09", surface: "F", holeId: 4, x: 279, y: 569 ,r:24},
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
    { artifact: "EY28", surface: "A", holeId: 1, x: 350, y: 296 ,r:10},
    { artifact: "EY28", surface: "A", holeId: 2, x: 460, y: 296 ,r:10},
    { artifact: "EY28", surface: "A", holeId: 3, x: 395, y: 639 ,r:15},
    { artifact: "EY28", surface: "A", holeId: 4, x: 229, y: 265 ,r:7},
    // EY28 - B面
    { artifact: "EY28", surface: "B", holeId: 1, x: 654, y: 444 ,r:9},
    { artifact: "EY28", surface: "B", holeId: 2, x: 485, y: 533 ,r:9},
    { artifact: "EY28", surface: "B", holeId: 3, x: 540, y: 647 ,r:9},
    { artifact: "EY28", surface: "B", holeId: 4, x: 260, y: 635 ,r:9},
    { artifact: "EY28", surface: "B", holeId: 5, x: 338, y: 533 ,r:9},
    { artifact: "EY28", surface: "B", holeId: 6, x: 335, y: 377 ,r:9},
    { artifact: "EY28", surface: "B", holeId: 7, x: 373, y: 215 ,r:9},
    { artifact: "EY28", surface: "B", holeId: 8, x: 498, y: 222 ,r:9},
    { artifact: "EY28", surface: "B", holeId: 9, x: 628, y: 343 ,r:9},
    { artifact: "EY28", surface: "B", holeId: 10, x: 488, y: 376 ,r:9},
    // EY28 - C面
    { artifact: "EY28", surface: "C", holeId: 1, x: 345, y: 282 ,r:7},
    { artifact: "EY28", surface: "C", holeId: 2, x: 320, y: 174 ,r:7},
    { artifact: "EY28", surface: "C", holeId: 3, x: 414, y: 173 ,r:7},
    { artifact: "EY28", surface: "C", holeId: 4, x: 447, y: 130 ,r:7},
    { artifact: "EY28", surface: "C", holeId: 5, x: 522, y: 210 ,r:7},
    { artifact: "EY28", surface: "C", holeId: 6, x: 504, y: 286 ,r:7},
    { artifact: "EY28", surface: "C", holeId: 7, x: 463, y: 453 ,r:7},
    { artifact: "EY28", surface: "C", holeId: 8, x: 492, y: 502 ,r:7},
    { artifact: "EY28", surface: "C", holeId: 9, x: 466, y: 663 ,r:15},
    { artifact: "EY28", surface: "C", holeId: 10, x: 392, y: 662 ,r:7},
    { artifact: "EY28", surface: "C", holeId: 11, x: 389, y: 475 ,r:15},
    // EY28 - D面
    { artifact: "EY28", surface: "D", holeId: 1, x: 256, y: 346 ,r:9},
    { artifact: "EY28", surface: "D", holeId: 2, x: 349, y: 228 ,r:9},
    { artifact: "EY28", surface: "D", holeId: 3, x: 424, y: 347 ,r:9},
    { artifact: "EY28", surface: "D", holeId: 4, x: 606, y: 368 ,r:9},
    { artifact: "EY28", surface: "D", holeId: 5, x: 622, y: 492 ,r:9},
    { artifact: "EY28", surface: "D", holeId: 6, x: 584, y: 658 ,r:9},
    { artifact: "EY28", surface: "D", holeId: 7, x: 436, y: 654 ,r:9},
    { artifact: "EY28", surface: "D", holeId: 8, x: 274, y: 654 ,r:9},
    { artifact: "EY28", surface: "D", holeId: 9, x: 226, y: 494 ,r:9},
    // EY28 - E面
    { artifact: "EY28", surface: "E", holeId: 1, x: 160, y: 382 ,r:17},
    { artifact: "EY28", surface: "E", holeId: 2, x: 270, y: 222 ,r:17},
    { artifact: "EY28", surface: "E", holeId: 3, x: 454, y: 220 ,r:17},
    { artifact: "EY28", surface: "E", holeId: 4, x: 635, y: 216 ,r:17},
    { artifact: "EY28", surface: "E", holeId: 5, x: 677, y: 391 ,r:17},
    { artifact: "EY28", surface: "E", holeId: 6, x: 644, y: 577 ,r:17},
    { artifact: "EY28", surface: "E", holeId: 7, x: 451, y: 566 ,r:17},
    { artifact: "EY28", surface: "E", holeId: 8, x: 256, y: 554 ,r:17},
    // EK30 - A面
    { artifact: "EK30", surface: "A", holeId: 1, x: 311, y: 585 ,r:15},
    { artifact: "EK30", surface: "A", holeId: 2, x: 290, y: 70 ,r:15},
    { artifact: "EK30", surface: "A", holeId: 3, x: 483, y: 589 ,r:15},
    { artifact: "EK30", surface: "A", holeId: 4, x: 408, y: 748 ,r:19},
    // EK30 - B面
    { artifact: "EK30", surface: "B", holeId: 1, x: 134, y: 520 ,r:7},
    { artifact: "EK30", surface: "B", holeId: 2, x: 201, y: 450 ,r:7},
    { artifact: "EK30", surface: "B", holeId: 3, x: 223, y: 346 ,r:7},
    { artifact: "EK30", surface: "B", holeId: 4, x: 299, y: 133 ,r:8},
    { artifact: "EK30", surface: "B", holeId: 5, x: 574, y: 172 ,r:8},
    { artifact: "EK30", surface: "B", holeId: 6, x: 445, y: 344 ,r:10},
    { artifact: "EK30", surface: "B", holeId: 7, x: 416, y: 372 ,r:7},
    { artifact: "EK30", surface: "B", holeId: 8, x: 552, y: 387 ,r:7},
    { artifact: "EK30", surface: "B", holeId: 9, x: 667, y: 445 ,r:7},
    { artifact: "EK30", surface: "B", holeId: 10, x: 526, y: 458 ,r:10},
    { artifact: "EK30", surface: "B", holeId: 11, x: 493, y: 477 ,r:7},
    { artifact: "EK30", surface: "B", holeId: 12, x: 490, y: 506 ,r:7},
    { artifact: "EK30", surface: "B", holeId: 13, x: 431, y: 520 ,r:8},
    { artifact: "EK30", surface: "B", holeId: 14, x: 528, y: 640 ,r:7},
    { artifact: "EK30", surface: "B", holeId: 15, x: 403, y: 542 ,r:9},
    { artifact: "EK30", surface: "B", holeId: 16, x: 354, y: 416 ,r:9},
    { artifact: "EK30", surface: "B", holeId: 17, x: 321, y: 434 ,r:10},
    // EK30 - C面
    { artifact: "EK30", surface: "C", holeId: 1, x: 412, y: 654 ,r:15},
    // EK30 - D面
    { artifact: "EK30", surface: "D", holeId: 1, x: 135, y: 494 ,r:7},
    { artifact: "EK30", surface: "D", holeId: 2, x: 172, y: 389 ,r:7},
    { artifact: "EK30", surface: "D", holeId: 3, x: 260, y: 251 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 4, x: 273, y: 193 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 5, x: 411, y: 249 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 6, x: 470, y: 123 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 7, x: 550, y: 253 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 8, x: 508, y: 379 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 9, x: 656, y: 399 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 10, x: 588, y: 510 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 11, x: 493, y: 656 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 12, x: 404, y: 662 ,r:9},
    { artifact: "EK30", surface: "D", holeId: 13, x: 222, y: 643 ,r:9},
    // EK30 - E面
    { artifact: "EK30", surface: "E", holeId: 1, x: 290, y: 325 ,r:14},
    { artifact: "EK30", surface: "E", holeId: 2, x: 500, y: 321 ,r:15},
    { artifact: "EK30", surface: "E", holeId: 3, x: 496, y: 547 ,r:14},
    { artifact: "EK30", surface: "E", holeId: 4, x: 396, y: 630 ,r:14},
    { artifact: "EK30", surface: "E", holeId: 5, x: 297, y: 545 ,r:14},
    // EK40 - A面
    { artifact: "EK40", surface: "A", holeId: 1, x: 296, y: 118 ,r:12},
    { artifact: "EK40", surface: "A", holeId: 2, x: 517.5, y: 380.5 ,r:20},
    // EK40 - B面
    { artifact: "EK40", surface: "B", holeId: 1, x: 197, y: 429 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 2, x: 237, y: 310 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 3, x: 239, y: 144 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 4, x: 276, y: 137 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 5, x: 536, y: 164 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 6, x: 610, y: 167 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 7, x: 428, y: 375 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 8, x: 587, y: 418 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 9, x: 509, y: 482 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 10, x: 446, y: 523 ,r:9},
    { artifact: "EK40", surface: "B", holeId: 11, x: 542, y: 650 ,r:7},
    { artifact: "EK40", surface: "B", holeId: 12, x: 322, y: 653 ,r:7},
    { artifact: "EK40", surface: "B", holeId: 13, x: 160, y: 533 ,r:7},
    { artifact: "EK40", surface: "B", holeId: 14, x: 365, y: 421 ,r:9},
    // EK40 - C面
    { artifact: "EK40", surface: "C", holeId: 1, x: 317, y: 203 ,r:11},
    { artifact: "EK40", surface: "C", holeId: 2, x: 508, y: 492 ,r:11},
    { artifact: "EK40", surface: "C", holeId: 3, x: 434, y: 638 ,r:14},
    { artifact: "EK40", surface: "C", holeId: 4, x: 385, y: 491 ,r:11},
    // EK40 - D面
    { artifact: "EK40", surface: "D", holeId: 1, x: 208, y: 308 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 2, x: 208, y: 273 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 3, x: 170, y: 226 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 4, x: 370, y: 275 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 5, x: 455, y: 139 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 6, x: 458, y: 276 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 7, x: 516, y: 393 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 8, x: 560, y: 481 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 9, x: 474, y: 632 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 10, x: 323, y: 636 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 11, x: 210, y: 633 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 12, x: 173, y: 565 ,r:9},
    { artifact: "EK40", surface: "D", holeId: 13, x: 124, y: 493 ,r:9},
    // EK40 - E面
    { artifact: "EK40", surface: "E", holeId: 1, x: 286, y: 362 ,r:14},
    { artifact: "EK40", surface: "E", holeId: 2, x: 490, y: 238 ,r:9},
    { artifact: "EK40", surface: "E", holeId: 3, x: 509, y: 356 ,r:14},
    { artifact: "EK40", surface: "E", holeId: 4, x: 502, y: 591 ,r:14},
    { artifact: "EK40", surface: "E", holeId: 5, x: 303, y: 599 ,r:14},

  ];

  return {
    holePositions,
    getHolePositions: (artifact, surface, holeId) =>
      holePositions.filter((pos) => pos.artifact === artifact && pos.surface === surface && pos.holeId == holeId),
  };
});