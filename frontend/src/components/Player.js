import React from "react";

import shipImages from "./../assets/spritesheet.png";

const ship_direction = {
  UP: {
    offsetX: -16,
    offsetY: -3,
    width: 23,
    height: 54,
  },
  RIGHT: {
    offsetX: -221,
    offsetY: -7,
    width: 53,
    height: 43,
  },
  DOWN: {
    offsetX: -456,
    offsetY: 0,
    width: 23,
    height: 59,
  },
  LEFT: {
    offsetX: -661,
    offsetY: -7,
    width: 53,
    height: 44,
  },
};

class Player extends React.Component {
  constructor(props) {
    super(props);

    this.images = shipImages;
    
    // angles of sprites
    this.angles = Array.apply(null, { length: 16 }).map(
      (_, i) => (360 / 16) * i
    ); 

    this.state = {
      locX: props.startX,
      locY: props.startY,
      direction: props.direction,
    };
  }

  style() {
    let res = {
    //   height: "50px",
    //   width: "50px",
    //   margin: "auto",
    border: 0,

    };

    return res;
  }

//   getBackground() {
//     const ship_dir = ship_direction[this.state.direction];
//     const bg = `url(${this.images}) ${ship_dir["offsetX"]}px ${ship_dir["offsetY"]}px`;
//     console.log(bg);
//     return bg;
//   }

  spriteStyle() {
    let ship_dir = ship_direction[this.state.direction];

    // const bg = `url(${this.images}) ${ship_dir["offsetX"]}px ${ship_dir["offsetY"]}px`;
    // console.log(bg);
    let res = {
      background: `url(${this.images}) ${ship_dir["offsetX"]}px ${ship_dir["offsetY"]}px`,
      backgroundRepeat: "no-repeat",
      backgroundColor: "transparent",
      display: "inline-block",
      width: `${ship_dir["width"]}px`,
      height: `${ship_dir["height"]}px`,
      transform: "scale(0.8,0.8)",
      position: "relative",
      border: "none",
      //margin: "auto",
      //top: "50%",
      //left: "50%",
      //width: "100%",
      //height: "auto",
    };

    return res;
  }

  render() {
    return (
      <div className="Player" style={this.spriteStyle()}></div>
    );
  }
}

export default Player;
