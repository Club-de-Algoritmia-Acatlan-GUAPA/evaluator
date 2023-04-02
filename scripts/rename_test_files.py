import os
import re
#for root, dirs, files in os.walk("../tests/sum_of_two_values/", topdown=False):
#    for name in files:
#        num = re.findall(r'\d+', name)[0]
#        input_type = 'input' in name 
#        if input_type:
#            os.rename('../tests/sum_of_two_values/' + name, f'input_{num}')
#        else:
#            os.rename('../tests/sum_of_two_values/' + name, f'output_{num}')
#

for root, dirs, files in os.walk(".", topdown=False):
    for name in files:
        input_type = 'input' in name 
        if input_type:
            os.rename(name, '../tests/sum_of_two_values/' + name + '.in')
        else:
            os.rename(name, '../tests/sum_of_two_values/' + name + '.out')
