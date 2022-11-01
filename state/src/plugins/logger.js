
import VueLogger from 'vuejs-logger'
import Vue from 'vue';


const isProduction = process.env.NODE_ENV === 'production';

Vue.use(VueLogger, {
  isEnabled: true,
  logLevel: isProduction ? 'error' : 'debug',
  stringifyArguments: false,
  showLogLevel: true,
  showMethodName: true,
  separator: '|',
  showConsoleColors: true
});
