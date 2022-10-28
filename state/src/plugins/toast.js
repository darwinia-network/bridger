import VuetifyToast from 'vuetify-toast-snackbar-ng'
import Vue from 'vue';


Vue.use(VuetifyToast, {
  x: 'right', // default
  y: 'bottom', // default
  color: 'primary', // default
  icon: 'mdi-information',
  iconColor: '', // default
  classes: ['body-2'],
  timeout: 3000, // default
  dismissable: true, // default
  multiLine: false, // default
  vertical: false, // default
  queueable: false, // default
  showClose: false, // default
  closeText: '', // default
  closeIcon: 'close', // default
  closeColor: '', // default
  slot: [], //default
  shorts: {
    custom: {
      color: 'purple'
    }
  },
  property: '$toast' // default
})
