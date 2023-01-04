RESULT=$(cat gltf_meshes | sed 's/.*"name" : "\(.*\)".*/#\1/g' | sed 's/^[^#].*//g' | sed 's/^#\(.*\)/\1/g' | sed -r 's/([A-Z])/_\L\1/g' | sed 's/^_\| _/\n/g'| sed '/^$/d') 
echo $RESULT | sed 's/ /\n/g' | awk '{print "\""$0"\": File (path: \"models_2.gltf#Mesh"NR-1"/Primitive0\"),"}'
echo ""
echo $RESULT | sed 's/ /\n/g'  | sed -r 's/(.*)/#[asset(key="\1")]\n pub \1 : Handle<Mesh>,\n/g' 