//
// Created by segransm on 4/10/22.
//

#define row_col_size                                                           
  3 void Rotate_ClockWise(int arr[row_col_size][row_col_size]) {               
    for (int i = 0; i < row_col_size; i++) {                                   
      for (int j = 0; j < row_col_size - i; j++) {                             
        int ptr = arr[i][j];                                                   
        arr[i][j] = arr[row_col_size - 1 - j][row_col_size - 1 - i];           
        arr[row_col_size - 1 - j][row_col_size - 1 - i] = ptr;                 
      }                                                                        
    }                                                                          
    for (int i = 0; i < row_col_size / 2; i++) {                               
      for (int j = 0; j < row_col_size; j++) {                                 
        int ptr = arr[i][j];                                                   
        arr[i][j] = arr[row_col_size - 1 - i][j];                              
        arr[row_col_size - 1 - i][j] = ptr;                                    
      }                                                                        
    }                                                                          
  }                                                                            
  int main() {                                                                 
    int arr[row_col_size][row_col_size] = {{5, 1, 4}, {9, 16, 12}, {2, 8, 9}}; 
    Rotate_ClockWise(arr);                                                     
    cout << "Rotation of a matrix by 90 degree in clockwise direction "        
            "without using any extra space is: \n";
    for (int i = 0; i < row_col_size; i++) {                                   
      for (int j = 0; j < row_col_size; j++) {                                 
        cout << arr[i][j] << " ";                                              
      }                                                                        
      cout << '\n';
    }                                                                          
    return 0;                                                                  
  }
