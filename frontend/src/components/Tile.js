import React from "react";

import seaImage from "./../assets/sea.png";
import windImage from "./../assets/wind_up.png";
import rockImage from "./../assets/rock_tile.png";
import whirlImage from "./../assets/cw_tl.png";

const Directions = {
    UP: 'u',
    DOWN: 'd',
    LEFT: 'l',
    RIGHT: 'r'
};

const TileTypes = {
    'c': {id: "cw_whirl", image: `url(${whirlImage})`},
    'x': {id: "cx_whirl", image: `url(${whirlImage})`},
    'w': {id: "wind", image: `url(${windImage})`},
    'r': {id: "rock", image: `url(${rockImage})`},
    's': {id: "sea", image: `url(${seaImage})`}
};

class Tile extends React.Component{
    constructor(props) {
        super(props);
        this.row_index = props.row_index;
        this.col_index = props.col_index;

        let first = props.tile_id[0];
        // console.log('tile id:' + props.tile_id)
        //console.log(first)
        let tile_info = TileTypes[first];
        this.type = tile_info['id'];
        this.image = tile_info['image'];
        // console.log('type: ' + this.type + ' image: ' + this.image);
        this.flip = false; // flip clockwise to counter-clockwise
        this.set_transform_degree(first, props.tile_id)
    }

    set_transform_degree(first, s) {
        let second = s[1];
        let rest = s.substr(2, 3);

        switch (first) {
            case 'c' || 'x':
                switch (rest) {
                    case 'tl':
                        this.transform_degree = 0;
                        break;
                    case 'tr':
                        this.transform_degree = 90;
                        break;
                    case 'bl':
                        this.transform_degree = 180;
                        break;
                    case 'br':
                        this.transform_degree = 270;
                        break;
                    default:
                        this.transform_degree = 0;
                        break;
                }
                break;
            case 'w':
                switch (second) {
                    case Directions['UP']:
                        this.transform_degree = 0;
                        break;
                    case Directions['DOWN']:
                        this.transform_degree = 180;
                        break;
                    case Directions['LEFT']:
                        this.transform_degree = 270;
                        break;
                    case Directions['RIGHT']:
                        this.transform_degree = 90;
                        break;
                    default:
                        this.transform_degree = 0;
                        break;
                }
                break;
            default:
                this.transform_degree = 0;
        }
    }

    style() {
        let res = {
            backgroundImage: this.image,
            // margin: 'auto',
            border: '1px solid',
            height: '40px',
            width: '40px',
            pointerEvents: 'auto',
            textAlign: 'center',
            // position: 'relative',
            cursor: 'pointer',
            transform: 'rotate(' + this.transform_degree + 'deg)'
        };

        return res;
    }

    render() {
        return (
            <div className="Tile" style={ this.style() }>
                {'(' + this.row_index + ',' +this.col_index + ')'} 
            </div>
        );
    }
}

export {TileTypes};
export default Tile;
