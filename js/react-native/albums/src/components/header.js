import React from 'react';
import {
  Text,
  View
} from 'react-native';

const Header = (props) => {
  const {textStyle, viewStyle} = styles;
  return (
    <View style={viewStyle}>
    <Text style={textStyle}>{props.textHeader}</Text>
    </View>
  );
};

const styles = {
  textStyle : {
    fontSize: 20,
  },
  viewStyle : {
    justifyContent: 'center',
    alignItems: 'center',
    height: 60,
    paddingTop: 15,
    backgroundColor: '#F8F8F8',
    borderBottomWidth: 1,
    borderColor: '#BDBDBD'
  }
};

export default Header;
