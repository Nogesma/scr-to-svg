use scr_to_svg::cube_puzzle::CubePuzzle;

#[test]
fn test_333() {
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("333");
        cp.apply_algorithm("U F' U2 F R2 B' U2 L2 R2 F D2 R2 U' B2 U' R B' F' L D2 U");

        assert_eq!(
            cp.image,
            [
                [[1, 5, 4], [1, 0, 4], [5, 1, 0]],
                [[1, 0, 0], [2, 1, 3], [2, 5, 3]],
                [[0, 0, 2], [0, 2, 3], [4, 4, 3]],
                [[0, 1, 4], [5, 3, 2], [3, 3, 2]],
                [[3, 3, 1], [2, 4, 0], [4, 4, 5]],
                [[5, 4, 2], [5, 5, 1], [1, 2, 5]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("333");
        cp.apply_algorithm("B F2 U2 B2 F2 R U2 L2 R U' B D2 L' D' L2 U2 B D' F2");

        assert_eq!(
            cp.image,
            [
                [[4, 2, 3], [2, 0, 0], [2, 1, 0]],
                [[3, 4, 2], [3, 1, 3], [2, 0, 5]],
                [[1, 5, 0], [5, 2, 1], [5, 1, 1]],
                [[5, 5, 0], [4, 3, 1], [0, 0, 4]],
                [[3, 0, 3], [2, 4, 3], [1, 5, 2]],
                [[4, 2, 1], [4, 5, 3], [4, 4, 5]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("333");
        cp.apply_algorithm("F U R U2 B U2 B' D2 B L2 D2 L2 D F U L' D' L B2");

        assert_eq!(
            cp.image,
            [
                [[2, 3, 1], [5, 0, 4], [3, 2, 3]],
                [[1, 0, 2], [4, 1, 1], [3, 2, 4]],
                [[4, 1, 0], [0, 2, 1], [0, 5, 1]],
                [[0, 2, 5], [0, 3, 5], [4, 5, 5]],
                [[1, 3, 5], [4, 4, 3], [0, 4, 4]],
                [[3, 1, 2], [0, 5, 2], [2, 3, 5]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("333");
        cp.apply_algorithm("U2 R F2 L D2 B2 F2 L2 U' L' U2 L2 D F U B' L2 U F2");

        assert_eq!(
            cp.image,
            [
                [[5, 5, 1], [4, 0, 2], [0, 2, 2]],
                [[1, 1, 3], [1, 1, 3], [2, 0, 4]],
                [[4, 1, 3], [0, 2, 0], [3, 0, 4]],
                [[0, 3, 3], [4, 3, 2], [1, 1, 2]],
                [[1, 5, 5], [5, 4, 4], [5, 3, 4]],
                [[5, 2, 2], [3, 5, 5], [0, 4, 0]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("333");
        cp.apply_algorithm("U' L2 U L' U2 D' F' B' R2 F' R B L2 U2 B D2 F2");

        assert_eq!(
            cp.image,
            [
                [[0, 2, 3], [3, 0, 2], [0, 0, 2]],
                [[1, 0, 4], [1, 1, 4], [2, 0, 5]],
                [[4, 2, 1], [5, 2, 1], [1, 4, 2]],
                [[3, 5, 3], [1, 3, 3], [4, 4, 3]],
                [[2, 5, 4], [0, 4, 1], [0, 4, 0]],
                [[5, 5, 5], [3, 5, 2], [1, 3, 5]]
            ]
        );
    }
}

#[test]
fn test_222() {
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("222");
        cp.apply_algorithm("U' R F' R2 F R U2 R' F' U2 R2");

        assert_eq!(
            cp.image,
            [
                [[5, 2,], [3, 0,],],
                [[5, 3,], [3, 0,],],
                [[2, 4,], [2, 1,],],
                [[0, 4,], [3, 4,],],
                [[0, 5,], [4, 1,],],
                [[1, 1,], [2, 5,],],
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("222");
        cp.apply_algorithm("R' U2 R' F' R U' R F' R' U R'");

        assert_eq!(
            cp.image,
            [
                [[5, 0], [4, 4]],
                [[0, 5], [3, 3]],
                [[1, 1], [0, 2]],
                [[2, 2], [3, 5]],
                [[1, 3], [4, 0]],
                [[4, 1], [2, 5]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("222");
        cp.apply_algorithm("U' R' F U' F' U2 R2 U2 R2 U' F");

        assert_eq!(
            cp.image,
            [
                [[2, 0], [3, 2]],
                [[2, 5], [1, 4]],
                [[5, 0], [1, 2]],
                [[4, 0], [3, 5]],
                [[3, 1], [4, 0]],
                [[4, 3], [1, 5]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("222");
        cp.apply_algorithm("R U' R' U F2 U' F U2 F R U");

        assert_eq!(
            cp.image,
            [
                [[2, 3], [0, 1]],
                [[0, 5], [0, 3]],
                [[2, 4], [2, 4]],
                [[4, 1], [3, 1]],
                [[3, 5], [4, 0]],
                [[1, 2], [5, 5]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("222");
        cp.apply_algorithm("R' U2 F' U' F R' U' F U2 F U'");

        assert_eq!(
            cp.image,
            [
                [[1, 1], [4, 3]],
                [[3, 0], [3, 0]],
                [[1, 5], [0, 0]],
                [[2, 2], [3, 4]],
                [[5, 2], [4, 1]],
                [[2, 4], [5, 5]]
            ]
        );
    }
}

#[test]
fn test_444() {
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("444");
        cp.apply_algorithm("D2 B2 R2 L B2 D2 R B2 F' L2 D2 R L2 D L' B D L' B2 Rw2 B2 U Rw2 Fw2 Uw2 L' D2 L Fw2 U R F2 B L2 Fw L2 B Uw Rw' F' Rw Fw' U2 Rw2");

        assert_eq!(
            cp.image,
            [
                [[3, 2, 3, 1], [5, 0, 0, 5], [4, 5, 0, 0], [3, 0, 4, 4]],
                [[1, 3, 4, 0], [2, 1, 3, 2], [0, 4, 1, 0], [5, 1, 3, 5]],
                [[4, 0, 2, 4], [4, 0, 2, 3], [1, 4, 1, 3], [3, 1, 5, 2]],
                [[5, 1, 2, 0], [5, 5, 2, 2], [1, 1, 3, 5], [4, 4, 0, 1]],
                [[5, 2, 3, 1], [4, 5, 2, 4], [3, 4, 3, 5], [3, 2, 1, 0]],
                [[2, 5, 1, 0], [1, 4, 3, 0], [5, 2, 5, 3], [2, 0, 4, 2]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("444");
        cp.apply_algorithm("D L B R' F2 R2 D2 L2 D2 R' B2 L F2 D' L' F' U' R' D' R2 Fw2 U2 Rw2 L Fw2 R2 U' R' U2 D' R Uw2 D' Fw' L2 U' Fw R2 Fw2 L' F2 Rw Uw' L2 Fw' Uw");

        assert_eq!(
            cp.image,
            [
                [[3, 0, 3, 5], [2, 3, 0, 4], [5, 0, 3, 4], [0, 5, 2, 0]],
                [[2, 1, 0, 0], [2, 2, 5, 4], [5, 1, 2, 1], [5, 0, 5, 2]],
                [[3, 2, 0, 1], [3, 2, 3, 4], [0, 0, 5, 4], [1, 5, 1, 4]],
                [[4, 0, 3, 4], [4, 0, 4, 2], [3, 2, 1, 4], [0, 1, 3, 5]],
                [[3, 4, 2, 5], [5, 4, 1, 1], [5, 1, 5, 1], [2, 1, 0, 2]],
                [[1, 1, 3, 3], [0, 4, 4, 2], [3, 3, 5, 2], [4, 5, 3, 1]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("444");
        cp.apply_algorithm("F R D2 F2 U2 R2 B2 U2 L2 D L' B' U F2 R' F B2 U' L2 Fw2 R B' Uw2 Rw2 R' U2 R2 B2 D2 Rw2 Fw2 B' Uw' F' Rw2 R2 Uw2 U2 Fw' R2 Uw Fw' Rw2 L2 Uw");

        assert_eq!(
            cp.image,
            [
                [[0, 2, 1, 5], [4, 2, 4, 4], [2, 2, 0, 4], [1, 5, 5, 0]],
                [[0, 3, 2, 3], [5, 0, 5, 0], [5, 1, 4, 4], [3, 2, 1, 2]],
                [[2, 4, 3, 4], [3, 1, 2, 3], [0, 0, 5, 0], [3, 2, 3, 3]],
                [[2, 0, 1, 4], [0, 1, 5, 2], [1, 1, 3, 4], [1, 0, 2, 1]],
                [[5, 1, 1, 2], [1, 0, 4, 3], [4, 4, 2, 3], [5, 5, 1, 4]],
                [[4, 0, 2, 1], [3, 3, 3, 5], [5, 3, 5, 0], [5, 5, 4, 0]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("444");
        cp.apply_algorithm("D2 F2 D F2 R2 D' F2 D2 B2 L R U F' L' U2 F2 L' B' L U2 Rw2 Fw2 U F' B U F' Rw2 D2 F' Uw2 R' U2 Rw F' L' D' B' Uw Rw' Fw U B2 R2 Uw");

        assert_eq!(
            cp.image,
            [
                [[2, 3, 5, 0], [2, 1, 5, 3], [1, 3, 2, 4], [2, 2, 1, 5]],
                [[4, 5, 5, 1], [1, 4, 3, 3], [0, 1, 4, 2], [1, 1, 5, 3]],
                [[0, 3, 1, 4], [0, 0, 3, 0], [4, 2, 4, 0], [3, 5, 0, 1]],
                [[0, 2, 4, 2], [3, 5, 3, 5], [1, 0, 0, 2], [2, 4, 2, 5]],
                [[4, 4, 1, 3], [0, 2, 5, 1], [5, 1, 2, 5], [0, 3, 4, 3]],
                [[5, 0, 3, 5], [4, 4, 5, 4], [2, 0, 1, 3], [1, 0, 2, 4]]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("444");
        cp.apply_algorithm("D' F U' L2 D U' L2 U R2 D F' U' R D' L2 F' L D' R' Rw2 U' Rw2 Uw2 Fw2 L' U L' D Rw2 Uw2 D Rw2 Fw U' L2 D Rw' Uw2 Fw2 L2 Fw' R U2");

        assert_eq!(
            cp.image,
            [
                [[4, 3, 3, 2], [4, 4, 2, 3], [5, 1, 0, 2], [1, 4, 2, 4]],
                [[0, 4, 5, 4], [4, 3, 0, 1], [1, 1, 2, 5], [2, 0, 1, 5]],
                [[0, 2, 2, 0], [5, 2, 3, 0], [1, 0, 5, 1], [1, 1, 5, 3]],
                [[2, 2, 0, 4], [3, 4, 1, 0], [0, 4, 5, 5], [1, 0, 4, 0]],
                [[5, 3, 4, 2], [3, 1, 3, 3], [5, 4, 2, 1], [3, 3, 4, 3]],
                [[3, 4, 0, 1], [2, 5, 0, 5], [0, 3, 5, 1], [5, 2, 2, 5]]
            ]
        );
    }
}
#[test]
fn test_555() {
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("555");
        cp.apply_algorithm("Uw2 Dw2 Lw Rw F2 B' D' F R U Dw2 Uw' Lw' Uw' D F' B2 Bw D2 Dw U2 B F2 Bw U' Rw2 Uw U2 Dw2 B2 Lw' Bw2 Uw' Bw' Rw D2 Dw' U L2 B' D Fw' Lw Fw2 D' Lw2 Fw2 Uw D2 R' Lw L' B Uw' D Rw' L' U2 Rw D'");

        assert_eq!(
            cp.image,
            [
                [
                    [0, 2, 4, 1, 0],
                    [5, 0, 5, 0, 4],
                    [2, 5, 0, 1, 0],
                    [4, 2, 3, 4, 4],
                    [3, 5, 0, 3, 5]
                ],
                [
                    [2, 1, 5, 1, 1],
                    [3, 1, 1, 0, 5],
                    [5, 4, 1, 0, 5],
                    [4, 1, 2, 4, 0],
                    [0, 3, 2, 5, 4]
                ],
                [
                    [2, 2, 0, 3, 5],
                    [2, 1, 2, 3, 0],
                    [4, 0, 2, 3, 3],
                    [2, 3, 0, 0, 0],
                    [1, 3, 5, 3, 2]
                ],
                [
                    [0, 1, 3, 2, 1],
                    [1, 2, 5, 5, 3],
                    [3, 4, 3, 3, 2],
                    [0, 5, 0, 3, 4],
                    [2, 2, 3, 1, 5]
                ],
                [
                    [3, 4, 1, 4, 1],
                    [0, 2, 3, 4, 3],
                    [1, 1, 4, 5, 1],
                    [0, 1, 4, 5, 1],
                    [4, 2, 1, 5, 4]
                ],
                [
                    [5, 2, 0, 5, 4],
                    [5, 3, 4, 2, 0],
                    [4, 2, 5, 1, 4],
                    [5, 5, 2, 4, 4],
                    [3, 0, 2, 1, 3]
                ]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("555");
        cp.apply_algorithm("Lw2 Uw' D2 R2 Lw' B Dw U F' Dw B2 U2 Rw' D2 Uw' Rw D' B2 D Uw2 L Uw2 L' Bw' Rw2 B2 L' Lw' F D2 Rw L Bw L2 Rw2 B2 Dw2 B' U F L2 Uw2 Bw' Uw2 Rw L Fw Rw D' U B' Fw2 Rw2 Lw2 U' Fw Lw D' R2 F'");

        assert_eq!(
            cp.image,
            [
                [
                    [0, 3, 5, 4, 3],
                    [1, 0, 2, 3, 2],
                    [3, 0, 0, 3, 1],
                    [5, 5, 0, 4, 5],
                    [3, 2, 4, 4, 0]
                ],
                [
                    [5, 2, 5, 5, 4],
                    [3, 1, 5, 4, 3],
                    [2, 1, 1, 4, 1],
                    [2, 5, 4, 0, 5],
                    [4, 3, 3, 5, 1]
                ],
                [
                    [3, 5, 2, 0, 2],
                    [4, 2, 4, 0, 5],
                    [5, 1, 2, 1, 5],
                    [1, 3, 1, 5, 4],
                    [1, 1, 1, 1, 2]
                ],
                [
                    [4, 2, 4, 4, 2],
                    [2, 2, 0, 2, 0],
                    [2, 2, 3, 2, 0],
                    [1, 1, 5, 3, 0],
                    [5, 3, 1, 0, 5]
                ],
                [
                    [3, 3, 2, 3, 1],
                    [2, 4, 3, 3, 3],
                    [0, 4, 4, 3, 3],
                    [4, 1, 2, 0, 0],
                    [0, 1, 4, 4, 2]
                ],
                [
                    [5, 0, 4, 1, 0],
                    [0, 1, 5, 2, 1],
                    [3, 5, 5, 0, 0],
                    [4, 5, 3, 4, 5],
                    [4, 2, 0, 0, 1]
                ]
            ]
        );
        {
            let mut cp: CubePuzzle = CubePuzzle {
                size: 0,
                cubie_size: 0,
                gap: 0,
                image: vec![],
            };

            cp.set_cube("555");
            cp.apply_algorithm("Dw F2 Lw2 F2 Bw2 Rw2 Dw' R Rw' Uw Dw' Bw2 Dw2 Lw2 Fw' D2 F2 R2 L Fw' Dw2 L B Fw R' Bw2 B F2 L2 Lw' B2 Uw2 U R2 B' F Bw R' B Dw2 Lw' Uw2 Fw B' Bw' L2 B' Rw' L R' B2 U2 Fw F' L R' Fw2 U' R2 Fw2");

            assert_eq!(
                cp.image,
                [
                    [
                        [5, 2, 1, 2, 2],
                        [2, 1, 4, 3, 3],
                        [0, 0, 0, 3, 2],
                        [1, 4, 0, 0, 3],
                        [4, 5, 5, 5, 2]
                    ],
                    [
                        [5, 0, 5, 5, 1],
                        [0, 2, 3, 5, 1],
                        [4, 0, 1, 2, 5],
                        [5, 2, 4, 1, 1],
                        [0, 0, 5, 3, 3]
                    ],
                    [
                        [4, 2, 4, 2, 1],
                        [4, 0, 1, 4, 4],
                        [2, 5, 2, 1, 1],
                        [2, 5, 5, 5, 3],
                        [1, 4, 2, 1, 3]
                    ],
                    [
                        [0, 4, 0, 1, 2],
                        [1, 0, 4, 1, 5],
                        [4, 2, 3, 2, 3],
                        [0, 0, 0, 4, 4],
                        [0, 1, 1, 2, 3]
                    ],
                    [
                        [2, 0, 0, 5, 5],
                        [3, 3, 5, 5, 3],
                        [3, 4, 4, 1, 0],
                        [3, 1, 5, 2, 0],
                        [5, 3, 4, 1, 3]
                    ],
                    [
                        [0, 4, 3, 2, 1],
                        [4, 2, 3, 4, 0],
                        [1, 1, 5, 2, 3],
                        [4, 3, 3, 3, 5],
                        [4, 0, 2, 5, 4]
                    ]
                ]
            );
        }
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("555");
        cp.apply_algorithm("B Fw' Uw' L2 Bw' Fw2 D U2 Rw' R' Lw2 B Rw' Dw' Lw' Bw R2 L2 Rw Fw2 Uw2 D R2 Lw D' Fw Uw' U' R F2 D Dw2 F' B' Fw D2 Fw' B2 Rw R' Lw' Dw R' B' Fw2 Lw2 D2 F2 Fw Lw2 Dw F Bw' B' L2 Rw Fw' B2 L' D2");

        assert_eq!(
            cp.image,
            [
                [
                    [1, 0, 4, 1, 4],
                    [3, 0, 0, 1, 0],
                    [1, 4, 0, 0, 4],
                    [0, 4, 0, 4, 5],
                    [0, 0, 5, 5, 3]
                ],
                [
                    [1, 2, 2, 0, 5],
                    [1, 2, 3, 5, 3],
                    [4, 3, 1, 4, 0],
                    [1, 5, 3, 4, 2],
                    [5, 3, 1, 2, 3]
                ],
                [
                    [4, 1, 0, 3, 2],
                    [2, 1, 3, 1, 4],
                    [0, 2, 2, 2, 3],
                    [5, 0, 5, 4, 1],
                    [1, 2, 2, 3, 2]
                ],
                [
                    [0, 2, 3, 5, 0],
                    [4, 2, 1, 3, 1],
                    [3, 1, 3, 1, 5],
                    [4, 2, 2, 3, 1],
                    [2, 5, 2, 2, 0]
                ],
                [
                    [5, 3, 4, 4, 4],
                    [4, 5, 2, 0, 4],
                    [0, 4, 4, 1, 3],
                    [3, 3, 4, 2, 0],
                    [4, 4, 5, 5, 1]
                ],
                [
                    [3, 1, 1, 0, 2],
                    [5, 0, 5, 5, 5],
                    [5, 5, 5, 0, 2],
                    [4, 3, 5, 1, 2],
                    [5, 3, 1, 0, 3]
                ]
            ]
        );
    }
    {
        let mut cp: CubePuzzle = CubePuzzle {
            size: 0,
            cubie_size: 0,
            gap: 0,
            image: vec![],
        };

        cp.set_cube("555");
        cp.apply_algorithm("F' U2 Fw' B Dw' L' Rw R2 Bw L2 Bw2 Uw' Dw Fw2 F2 U2 Dw2 Uw' F' Dw L Bw' Dw F' Dw' Bw R2 L2 U B2 Dw2 Lw F2 B U Uw Dw F' Lw' R2 Fw Bw2 Uw2 Rw U' R Uw Bw' Lw Bw Fw2 Dw2 B' L' Bw R' Fw2 B Lw2 U'");

        assert_eq!(
            cp.image,
            [
                [
                    [3, 3, 3, 4, 0],
                    [3, 0, 1, 1, 5],
                    [3, 5, 0, 4, 0],
                    [4, 3, 4, 4, 2],
                    [4, 4, 5, 5, 0]
                ],
                [
                    [0, 2, 3, 0, 2],
                    [5, 0, 0, 2, 0],
                    [2, 3, 1, 3, 5],
                    [1, 0, 3, 5, 1],
                    [3, 5, 2, 5, 5]
                ],
                [
                    [5, 1, 4, 0, 4],
                    [2, 1, 1, 2, 5],
                    [4, 4, 2, 5, 1],
                    [1, 4, 1, 5, 3],
                    [1, 0, 0, 0, 2]
                ],
                [
                    [5, 4, 1, 2, 1],
                    [3, 2, 2, 3, 3],
                    [3, 4, 3, 5, 5],
                    [3, 5, 2, 0, 3],
                    [1, 5, 1, 0, 3]
                ],
                [
                    [2, 4, 4, 1, 3],
                    [2, 4, 1, 3, 2],
                    [0, 0, 4, 0, 0],
                    [3, 3, 2, 1, 1],
                    [5, 5, 5, 2, 2]
                ],
                [
                    [1, 1, 4, 0, 4],
                    [4, 4, 2, 5, 2],
                    [2, 3, 5, 5, 2],
                    [4, 2, 0, 1, 4],
                    [4, 1, 1, 0, 0]
                ]
            ]
        );
    }
}
