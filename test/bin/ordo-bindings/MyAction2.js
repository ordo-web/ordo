export const increment = (payload) => {
   return {
      ident: 'MyAction2',
      action: {
         type: 'INCREMENT',
         payload: payload
      }
   }
};

export const decrement = () => {
   return {
      ident: 'MyAction2',
      action: {
         type: 'DECREMENT',
      }
   }
};

