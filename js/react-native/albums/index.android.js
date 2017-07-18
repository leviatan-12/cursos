//Import a library to help create the Component

import React from 'react';
import {
  AppRegistry
} from 'react-native';
import Header from './src/components/header';

//Create a Component
const App = () => {
  return (
    <Header textHeader={"Albums!"}/>
 );
};

//render it to the device
AppRegistry.registerComponent('albums', () => App);
