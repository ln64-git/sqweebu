import { useThemeColor } from "@/config/themes";
import React from "react";

export default function ResponseMessage() {
  const textPrimary = useThemeColor("textPrimary");

  return (
    <div style={{ color: textPrimary }} className="mx-2 my-2 mb-8 text-sm ">
      {words}
    </div>
  );
}

let words =
  "  Consequatur praesentium tempora quos earum? Blanditiis magni cum cupiditate vitae doloremque libero excepturi! Sint, magni soluta accusantium similique distinctio et voluptate sit suscipit ipsum nemo nulla atque quidem. Aliquid, dolorum?  Rerum id recusandae saepe a sunt et, in asperiores perspiciatis aspernatur iusto corrupti fugit, dolores cum unde. Odit molestias assumenda minus quae quos, natus nemo quis distinctio dolorem temporibus earum.   Illum natus dignissimos asperiores eaque aperiam incidunt aspernatur hic quaerat dolorem culpa autem ipsam commodi eum placeat quos veritatis, ab molestias, nostrum ducimus error impedit laudantium cumque! Voluptatem, explicabo. Doloribus?  Temporibus, aliquam. Est, totam? Atque sequi fugiat, officia, deserunt laborum corporis cupiditate culpa saepe iusto incidunt consequatur blanditiis exercitationem cum architecto beatae pariatur? Error at odit iusto quia veritatis dolorem!  Repellendus illum deserunt ducimus eos corporis minima quaerat dicta autem hic adipisci obcaecati ipsum natus optio, eum excepturi quia quam nostrum cupiditate rerum delectus dolores impedit quidem at! Nulla, dolores!  Dolor numquam suscipit beatae deleniti, odit quis! Architecto tenetur veniam, explicabo aliquam aliquid similique corporis obcaecati corrupti expedita sapiente ut doloribus, nostrum molestiae beatae ratione. Perspiciatis tempora quasi accusamus magnam.";
