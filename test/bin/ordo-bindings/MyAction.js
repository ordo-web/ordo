export const increment = (payload) => {
   return {
      ident: 'MyAction',
      action: {
         type: 'INCREMENT',
         payload: payload
      }
   }
};

export const decrement = () => {
   return {
      ident: 'MyAction',
      action: {
         type: 'DECREMENT',
      }
   }
};

