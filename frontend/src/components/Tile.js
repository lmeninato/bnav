import React from "react";

import seaImage from "./../assets/sea.png";

// const seaImage = require('./../assets/sea1.png');

const TileTypes = {
    CW: "clockwise",
    CCW: "counter-clockwise",
    WIND: "wind",
    ROCK: "rock",
    SEA: "sea"
};

class Tile extends React.Component{
    constructor(props) {
        super(props);
        switch (props.tiletype) {
            // todo all cases
            default:
                // assume it's a sea tile for now
                // console.log(seaImage)
                this.type = props.tiletype
                this.image = `url(${seaImage})`;
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
            // position: 'relative',
            cursor: 'pointer'
        };

        // console.log(this.image)
        return res;
    }

    render() {
        return (
            <div className="Tile" style={ this.style() }>
                
            </div>
        );
    }
}

export {TileTypes};
export default Tile;
