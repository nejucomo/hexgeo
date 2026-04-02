# Geometry

The [geom](crate::geom) module provides geometric support for modeling hexagons in 2D cartesian space. There are two _orientations_ supported by this crate: "pointy top" and "flat top". Applications can either hard code an orientation using the constants in [geom::pointy_top](crate::geom::pointy_top) / [geom::flat_top](crate::geom::flat_top), or dynamically switch at runtime using [geom::HexOrientation](crate::geom::HexOrientation).
